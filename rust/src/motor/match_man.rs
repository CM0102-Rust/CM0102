use super::*;

/// Assigns the `n` people in `pessoas` to `m` positions (+ `x` extras) and writes each position's person to `saida`,
/// scoring each (person, position) pair (`lineup_slot_score`: position code, mode, club, staff). With fewer people than positions
/// each free person takes the position where he beats the holder; with more, each position takes the best and
/// duplicates are resolved by least loss; extras get the leftovers recursively (mode | 0x200); finally pairs are
/// swapped while the total improves. Returns 1, or 0 with no people or positions [inference: picking the line-up].
#[allow(clippy::too_many_arguments)]
impl MatchManager {
    /// Match player k (0..20) of the manager's side: the 20-player block at `match_manager::PLAYERS`, 0x1be bytes each.
    pub(crate) unsafe fn player(self, k: usize) -> MatchPlayer {
        MatchPlayer(self.players().wrapping_add(k.wrapping_mul(MATCH_PLAYER_SIZE)))
    }
}

pub(crate) unsafe extern "C" fn pick(
    persons: usize, a2: u32, codes: usize, a4: u32, a5: u32, out: usize, mode: u32, club: usize, staff: usize,
    a10: u32, _a11: u32, mut marks: usize, mut bytes: usize, a14: u32, mut curtos: usize,
) -> u8 {
    let staff = MatchManager(staff);
    let club = Club(club);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let rating_fn: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let recursiva: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let n = a2 as u8 as i8 as i32;
    let m = a4 as u8 as i8 as i32;
    let x = a5 as u8 as i8 as i32;
    let tot = m + x;
    let pes = |k: i32| read_usize(persons + 4 * k as usize);
    let sai = |i: i32| (out + 4 * i as usize) as *mut usize;
    let (mut al12, mut al13, mut al15) = (false, false, false);
    if marks == 0 {
        marks = if tot != 0 { exe::crt_calloc(tot as u32, 8) } else { 0 };
        al12 = true;
    }
    if bytes == 0 {
        bytes = if tot != 0 { exe::crt_calloc(tot as u32, 1) } else { 0 };
        al13 = true;
    }
    if curtos == 0 {
        curtos = if tot != 0 { exe::crt_calloc(tot as u32, 2) } else { 0 };
        al15 = true;
    }
    let liberar_param = |free: unsafe extern "C" fn(usize)| {
        if al13 && bytes != 0 {
            free(bytes);
        }
        if al12 && marks != 0 {
            free(marks);
        }
        if al15 && curtos != 0 {
            free(curtos);
        }
    };
    if n < 1 {
        for i in 0..tot {
            *sai(i) = 0;
        }
        liberar_param(free);
        return 0;
    }
    let jog = exe::crt_calloc(n as u32, 4);
    let jg = |k: i32| read_usize(jog + 4 * k as usize);
    if !staff.is_null() {
        for k in 0..n {
            let mut j = staff.players();
            let mut found = 0usize;
            for _ in 0..20 {
                if read_usize(j + match_player::STAFF) == pes(k) {
                    found = j;
                    break;
                }
                j += 0x1be;
            }
            write_usize(jog + 4 * k as usize, found);
        }
    }
    let mut lc = 0u32;
    if mode & 0x400000 != 0 && !club.is_null() {
        let p = club.manager().addr();
        let q = read_i32(G_STAFF_COUNT);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && read_i8(p + staff::CLASSIFICATION) != 0 {
            lc = exe::human_manager_get_saved_lineup(read_usize(G_HUMAN_MANAGER), p);
        }
    }
    let used = exe::crt_calloc(n as u32, 1);
    let us = |k: i32| (used + k as usize) as *mut i8;
    for i in 0..tot {
        *sai(i) = 0;
    }
    if m == 0 && x == 0 {
        free(used);
        liberar_param(free);
        free(jog);
        return 0;
    }
    // the rating of `pessoa` (with player `j`) in slot `i`
    let rating = |person: Staff, j: usize, i: i32| -> i32 {
        let i = i as usize;
        rating_fn(
            person.addr(), person.player().addr(), j, read_u16(codes + 2 * i) as u32, mode, club.addr(), staff.addr(), a10, u32::MAX,
            read_u32(marks + 8 * i), read_u32(marks + 8 * i + 4), read_u8(bytes + i) as u32, a14,
            read_u16(curtos + 2 * i) as u32, lc,
        )
    };
    let soltar = |target: usize| {
        for kk in 0..n {
            if pes(kk) == target {
                *us(kk) = 0;
            }
        }
    };
    if n < m {
        // fewer people than slots: each free person looks for the best slot
        let mut colocadas = 0i8;
        let mut laps = 0;
        while (colocadas as i32) < n && laps < 10000 {
            laps += 1;
            for k in 0..n {
                if *us(k) != 0 {
                    continue;
                }
                let cand = pes(k);
                if cand == 0 || read_usize(cand + staff::PLAYER) == 0 {
                    continue;
                }
                let (mut melhor, mut best_rating) = (-1i32, 0i32);
                for i in 0..m {
                    let r = rating(Staff(cand), jg(k), i);
                    if melhor == -1 || r > best_rating {
                        let ocupante = *sai(i);
                        if ocupante == 0 || rating(Staff(ocupante), jg(k), i) < r {
                            melhor = i;
                            best_rating = r;
                        }
                    }
                }
                if melhor > -1 {
                    let ocupante = *sai(melhor);
                    if ocupante != 0 {
                        for kk in 0..n {
                            if pes(kk) == *sai(melhor) {
                                *us(kk) = 0;
                                colocadas = colocadas.wrapping_sub(1);
                            }
                        }
                    }
                    *sai(melhor) = cand;
                    colocadas = colocadas.wrapping_add(1);
                    *us(k) = 1;
                }
            }
        }
        for i in 0..m {
            let ocupante = *sai(i);
            if ocupante == 0 {
                continue;
            }
            let f = read_i8(read_usize(ocupante + staff::PLAYER) + player_sheet::GOALKEEPER);
            let exit = if read_u8(codes + 2 * i as usize) & 1 != 0 { f < 10 } else { f == 20 };
            if exit {
                soltar(ocupante);
                *sai(i) = 0;
            }
        }
    } else {
        // more people: each slot takes the best; duplicates are resolved later
        let mut conflitos = 0i8;
        let mut best_rating = 0i32;
        for i in 0..m {
            let (mut melhor, mut mk) = (0usize, 0i32);
            for k in 0..n {
                let cand = pes(k);
                if cand == 0 || read_usize(cand + staff::PLAYER) == 0 {
                    continue;
                }
                let r = rating(Staff(cand), jg(k), i);
                if melhor == 0 || r > best_rating {
                    melhor = cand;
                    best_rating = r;
                    mk = k;
                }
            }
            if melhor != 0 {
                *us(mk) = (*us(mk)).wrapping_add(1);
                if *us(mk) > 1 {
                    conflitos = conflitos.wrapping_add(1);
                }
            }
            *sai(i) = melhor;
        }
        let mut smaller_loss = 10000i32;
        let mut laps = 0;
        let mut k2_current = 0i32;
        while conflitos > 0 && laps < 10000 {
            laps += 1;
            let (mut vaga_t, mut k2_t) = (-1i32, -1i32);
            for k in 0..n {
                let mut swap = 0usize;
                let cand = pes(k);
                if m > 0 {
                    for j in 0..m {
                        if cand == 0 || *us(k) <= 1 {
                            break;
                        }
                        if cand != *sai(j) {
                            continue;
                        }
                        let r_current = rating(Staff(cand), jg(k), j);
                        let (mut outra, mut outra_nota) = (0usize, 0i32);
                        for k2 in 0..n {
                            let o = Staff(pes(k2));
                            if o.is_null() || o.player().is_null() || o.addr() == cand {
                                continue;
                            }
                            let r = rating(o, jg(k2), j);
                            if (outra == 0 || r > outra_nota) && (r < r_current || *us(k2) == 0) {
                                outra_nota = r;
                                outra = o.addr();
                                k2_current = k2;
                            }
                        }
                        if swap == 0 || r_current - outra_nota < smaller_loss {
                            vaga_t = j;
                            swap = outra;
                            k2_t = k2_current;
                            smaller_loss = (r_current - outra_nota) as i16 as i32;
                        }
                    }
                    if swap != 0 {
                        *us(k2_t) = (*us(k2_t)).wrapping_add(1);
                        if *us(k2_t) > 1 {
                            conflitos = conflitos.wrapping_add(1);
                        }
                        *sai(vaga_t) = swap;
                        *us(k) = (*us(k)).wrapping_sub(1);
                        conflitos = conflitos.wrapping_sub(1);
                    }
                }
            }
        }
    }
    // the extras: the people left over, via 0x6e2000 itself
    if x != 0 {
        let mut rest = [0usize; 128];
        let mut c = 0usize;
        for k in 0..n {
            if *us(k) == 0 {
                rest[c] = pes(k);
                c += 1;
            }
        }
        recursiva(
            rest.as_ptr() as usize, c as u32, codes + 2 * m as usize, x as u32, 0, out + 4 * m as usize, mode | 0x200, club.addr(), staff.addr(), a10, 0,
            marks, bytes, a14, curtos + 2 * m as usize,
        );
    }
    // pair swaps while the sum improves
    let player_of = |p: usize| -> usize {
        if staff.is_null() {
            return 0;
        }
        let mut j = staff.players();
        for _ in 0..20 {
            if read_usize(j + match_player::STAFF) == p {
                return j;
            }
            j += 0x1be;
        }
        0
    };
    let mut laps = 0;
    loop {
        let mut changed = false;
        let mut i = 0;
        while i < m && !changed {
            let a = Staff(*sai(i));
            if !a.is_null() && !a.player().is_null() {
                let ja = player_of(a.addr());
                let a_i = rating(a, ja, i);
                let mut j = i + 1;
                while j < tot && !changed {
                    let b = Staff(*sai(j));
                    if !b.is_null() && !b.player().is_null() {
                        let jb = player_of(b.addr());
                        let b_i = rating(b, jb, i);
                        let trocar = if j < m {
                            let a_j = rating(a, ja, j);
                            let b_j = rating(b, jb, j);
                            b_j.wrapping_add(a_i) < a_j.wrapping_add(b_i)
                        } else {
                            a_i < b_i
                        };
                        if trocar {
                            changed = true;
                            let t = *sai(j);
                            *sai(j) = *sai(i);
                            *sai(i) = t;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        laps += 1;
        if !changed || laps >= 100 {
            break;
        }
    }
    free(used);
    free(jog);
    liberar_param(free);
    1
}

/// the stance the staff ask for (1 all-out attack … 5 park the bus). Starts from the sides' strength (with the engine
/// running, `engine::MATCH_RUNNING`, the sum of the `perceived_ability` scores of those on the pitch × 50 / 11; otherwise the +0x80 reputation),
/// possession (`engine::ZONE_LOW_TICKS`/`engine::ZONE_MIDDLE_TICKS`/`engine::ZONE_HIGH_TICKS`), time (`engine::CLOCK`), sendings-off (+0x1ad, `exe::cm3_match_engine_count_players_on_pitch`) and the aggregate score
/// (`exe::cm3_match_engine_get_goal_margin`); adjusts for the league (table positions through `exe::comp_stats_find_club_entry` and dates through `exe::date_days_since`), the manager (+0xe)
/// and the end of the match (`engine::PERIOD_END` = 0x3de). Stores the factor in engine`engine::STANCE_FACTOR`[side] (10 and −1 at the extremes);
/// with the engine running and the factor within 0.5 of the stored one, keeps the current stance (+0x1ab)
/// Original: `stance` (0x6df460, match_man.cpp (~3 KB), thiscall).
pub(crate) unsafe extern "thiscall" fn stance(c: usize) -> u8 {
    let c = MatchManager(c);
    let u = |p: usize, o: usize| read_usize(p + o);
    let i = |p: usize, o: usize| read_i32(p + o);
    let w = |p: usize, o: usize| read_i16(p + o) as i32;
    let m = Engine(u(c.addr(), match_manager::ENGINE));
    let side = c.side();
    let lu = side as isize as usize;
    let (clube_a, clube_b) = (u(c.addr(), match_manager::CLUB), u(c.addr(), match_manager::OPPONENT));
    let (rep_a, rep_b) = (w(clube_a, club::REPUTATION), w(clube_b, club::REPUTATION));
    let rodando = m.match_running() != 0;
    // each side’s strength (only the 16 bits count, as in the original)
    let (mut own, mut other) = (rep_a, rep_b);
    if rodando {
        let mut sum = [0i32; 2];
        for (l, s) in sum.iter_mut().enumerate() {
            for k in 0..20 {
                let j = m.player(l as i8, k);
                if j.position() >= 0 {
                    *s = s.wrapping_add(exe::match_man_judge_player_ability(u(c.addr(), match_manager::MANAGER), u(j.addr(), match_player::STAFF), clube_a, (2) as i8, (u32::MAX) as i16) as i32);
                }
            }
        }
        let (a, b) = if side == 0 { (sum[0], sum[1]) } else { (sum[1], sum[0]) };
        own = a.wrapping_mul(50) / 11;
        other = b.wrapping_mul(50) / 11;
    }
    let (mut x11, mut x12, mut x14, mut x16, mut x17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    exe::cm3_match_engine_get_goal_margin(m, side, &mut x11, &mut x12, &mut x16, &mut x17, &mut x14, 0);
    let own = own as i16 as i32;
    let other = other as i16 as i32;
    let f = (own - other) as f64 / ((1.5 - own as f64 * 0.0001) * 6000.0) + 1.0;
    // a posse
    let (p1, p2, p3) = (w(m.addr(), engine::ZONE_LOW_TICKS), w(m.addr(), engine::ZONE_MIDDLE_TICKS), w(m.addr(), engine::ZONE_HIGH_TICKS));
    let tot = p1 + p2 + p3;
    let posse_b = if tot != 0 { (p3 * 100 / tot) as i8 } else { 0 };
    let posse_a = if tot != 0 { (p1 * 100 / tot) as i8 } else { 0 };
    let r = if side == 0 && posse_a != 0 {
        Some(posse_b as f64 / posse_a as f64)
    } else if side == 1 && posse_b != 0 {
        Some(posse_a as f64 / posse_b as f64)
    } else {
        None
    };
    let r = match r {
        Some(q) if q > 1.0 => q + q - 1.0,
        Some(q) => q * 1.29 - 0.29,
        None => f,
    };
    let r = if 0.05 > r { 0.05 } else if 6.0 < r { 6.0 } else { r };
    let mut t = ((w(m.addr(), engine::CLOCK) << 2) / 99) as i8;
    if t > 20 {
        t = 20;
    }
    let t = t as f64;
    let k = read_i8(m.addr() + (0x1ad + lu));
    let g = f * (25.0 - t) + t * r;
    let mut h = g * 0.04 * (1.0 - k as f64 * 0.07);
    if !(h > 0.0) {
        h = 0.01;
    }
    if x14 != 0 {
        if side == 1 && x11 != 0 {
            x11 = x11.wrapping_add(2);
        } else if side == 0 {
            if x12 != 0 {
                x12 = x12.wrapping_add(2);
            } else {
                x11 = x11.wrapping_add(1);
            }
        }
    }
    // the sent-off players: 0x6cbbb0 counts the active ones on each side
    let mut x13 = 0i8;
    if rodando {
        let b = exe::cm3_match_engine_count_players_on_pitch(m, (side == 0) as u32);
        let a = exe::cm3_match_engine_count_players_on_pitch(m, side as u8 as u32);
        x13 = (a as i8).wrapping_sub(b as i8);
        if x13 != 0 {
            x11 = x11.wrapping_add(x13.wrapping_mul(-2));
        }
    }
    let tt = (w(m.addr(), engine::CLOCK) / 11) as f64;
    if x14 == 0 && x17 == 0 && x11 as i32 + x12 as i32 == 0 {
        let (d, e) = (read_i8(m.addr() + (0x1ad + (side == 0) as usize)), read_i8(m.addr() + (0x1ad + lu)));
        if d > e {
            h *= tt * 0.014925373134328358 + 1.25;
        } else if d < e {
            h *= 0.75;
        }
    }
    h *= 1.0 - (x11 as i32 - x12 as i32) as f64 * 0.1;
    if u(clube_a, club::NATION) != u(clube_b, club::NATION) {
        h *= 0.9;
    } else {
        let game = u(c.addr(), match_manager::FIXTURE);
        let p14 = u(game, fixture::COMPETITION);
        if p14 != 0 {
            h += h;
            let comp = if p14 == u(clube_a, club::DIVISION) { u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(game, 0)) } else { 0 };
            if comp != 0 && u(comp, 8) != 0 {
                // the league: who is above in the table and how far behind
                let e = exe::comp_stats_find_club_entry(u(comp, 8), clube_a);
                if e != 0 {
                    let mut x18 = 0f32;
                    let mut done = false;
                    if i(e, 0x55) > 1 && exe::date_days_since(G_CURRENT_DATE, u(e, 0x61) as u32, u(e, 0x65) as u32) < 30 {
                        let (a, b) = (i(e, 0x55).wrapping_neg(), i(e, 0x2d).wrapping_neg().wrapping_shl(1));
                        x18 = (if a < b { a } else { b }) as f32;
                        done = true;
                    }
                    if !done && i(e, 0x41) > 1 && exe::date_days_since(G_CURRENT_DATE, u(e, 0x4d) as u32, u(e, 0x51) as u32) < 10 {
                        let (a, b) = (i(e, 0x41), i(e, 0x19).wrapping_shl(1));
                        x18 = (if a > b { a } else { b }) as f32;
                    }
                    x18 = ((x18 as f64).abs() * x18 as f64) as f32;
                    if k == 0 && 0.0f32 > x18 {
                        x18 = 0.0;
                    }
                    let mut base = h;
                    let pe = read_u8(e + comp_stats_club::FINAL_POSITION);
                    if pe < 5 && (x11 as i32 - 3) < x12 as i32 {
                        let e2 = exe::comp_stats_find_club_entry(u(u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(game, 0)), 8), clube_b);
                        if e2 != 0 && *((e2 + comp_stats_club::FINAL_POSITION) as *const u8) > pe {
                            let comp = u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(game, 0));
                            let cx = w(comp, 0x3e);
                            let mut x30 = if cx > 0 { 22.0 / cx as f64 } else { 1.0 };
                            let ax = w(u(game, fixture::COMPETITION), 0x69);
                            if ax >= 10 {
                                let lim = ax * 500;
                                if rep_a > lim {
                                    x30 *= ((rep_a - lim) as f64 + (rep_a - 5000) as f64) * 0.0002 + 1.0;
                                }
                            }
                            let p = game_pow(x30, 1.5) * (read_u8(e2 + comp_stats_club::FINAL_POSITION) as i32 - pe as i32) as f64;
                            let p3 = p * p * p;
                            let q = p3 / ((k as i32 * k as i32) as f64 * 50.0 + 250.0) + 1.0;
                            let q = if 1.0 > q { 1.0 } else if 10.0 < q { 10.0 } else { q };
                            base = q * h;
                        }
                    }
                    let z = x18 as f64 * 0.013333333333333334 + 1.0;
                    let z = if 0.25 > z { 0.25 } else if 5.0 < z { 5.0 } else { z };
                    h = base * z;
                }
            }
        }
    }
    let sum = x11 as i32 + x12 as i32;
    if sum > 6 {
        h *= 0.825;
    } else if sum > 4 {
        h *= 0.875;
    }
    let kb = k;
    if kb != 0 || x13 < 0 {
        let pressure = (rep_b > 7500 && rep_b - rep_a > 2000) || (rep_b > 8500 && rep_b - rep_a > 750) || (rep_b > 8000 && x11 as i32 - 1 > x12 as i32);
        if pressure {
            let d = x12 as i32 - x11 as i32;
            if (d < 2 && w(m.addr(), engine::PERIOD_END) == 0x1ef) || x12 <= x11 {
                h *= 0.3;
            }
        }
    }
    let d = x11 as i32 - x12 as i32;
    h *= if kb == 0 { 1.0 - d as f64 * 0.1 } else { 0.967 - (kb as i32 + 2 * d) as f64 * 0.067 };
    if x11 > x12 || (x11 == x12 && (x16 != 0 || side == 1)) {
        let a = if 1.0 > tt { 1.0 } else { tt };
        h *= if 1.0 < 30.0 / a { 1.0 } else { 30.0 / a };
    } else if x11 < x12 || kb == 0 {
        let v = tt * 0.03333333333333333;
        h *= if 1.0 > v { 1.0 } else { v };
    }
    let d2 = x12 as i32 - x11 as i32;
    if d2 > 2 && d2 < 6 {
        h *= 1.01 - d2 as f64 * 0.2;
    }
    let ra_greater = w(clube_a, club::REPUTATION) > w(clube_b, club::REPUTATION);
    if ((sum < 5 && x13 > -1 && x11 == x12 && x14 == 0 && ra_greater) || x13 > 0) && !(h > 1.0) {
        h = 1.0;
    }
    if sum < 4 && x13 > -1 && x11 as i32 - 2 >= x12 as i32 && ra_greater && !(h > 1.15) {
        h = 1.15;
    }
    let manager = NonPlayer(u(c.addr(), match_manager::MANAGER_SHEET));
    if !manager.is_null() && x13 > -1 {
        let e = manager.attacking() as i32;
        let mut f1 = (1.0 + (e - 10) as f64 * 0.06666666666666667) * h;
        let e2 = game_pow(e as f64, 2.0);
        if f1 < e2 * 0.005 {
            f1 = e2 * 0.0025 + f1 * 0.5;
        }
        let hm1 = h - 1.0;
        h = if f1 < hm1 { hm1 } else { f1 };
    }
    if (rep_b > 7500 || x13 < 0) && kb != 0 && d.abs() < 2 && (x11 >= x12 || tt < 75.0 || sum > 3) {
        let g = tt * 0.02857142857142857 + 0.2;
        if !(h < g) {
            h = g;
        }
    }
    if x17 != 0 && x14 == 0 && x16 == 0 && (x11 == x12 || x11 as i32 + 1 == x12 as i32) && kb == 0 {
        let g = (if tt < 45.0 { tt } else { 45.0 }) * 0.02857142857142857 + 0.2;
        if !(h > g) {
            h = g;
        }
    }
    if x12 > 1 && x11 > 1 {
        let cai = if x14 == 0 && x17 != 0 { (tt + 90.0) / sum as f64 } else { tt / sum as f64 };
        if cai < 18.0 {
            h *= 0.3;
        }
    }
    let factor = (m.addr() + engine::STANCE_FACTOR + 4 * lu) as *mut f32;
    let team = w(m.addr(), engine::PERIOD_END);
    if tt > 85.0 - h * 10.0 && team == 0x3de && (x11 as i32 + 1 == x12 as i32 || (x17 != 0 && x14 == 0 && x16 == 0 && x11 == x12)) {
        *factor = 10.0;
        return 1;
    }
    if sum < 5 && tt > 82.0 - h * 10.0 && team == 0x3de && x11 == x12 && x16 == 0 && (h > 1.0 || read_i8(m.addr() + (0x1ab + lu)) == 1) {
        *factor = 10.0;
        return 1;
    }
    if h < 1.4 && kb != 0 && d > 1 {
        h = 0.2;
    }
    let retranca = (d > 3 && h > 1.0)
        || (tt > 30.0 && x11 >= x12 && 10.0 / h < ((d + 1) * (d + 1)) as f64 && clube_a != u(clube_b, club::RIVAL1) && clube_a != u(clube_b, club::RIVAL2) && clube_a != u(clube_b, club::RIVAL3));
    if retranca {
        *factor = -1.0;
        return 5;
    }
    if rodando && (h - *factor as f64).abs() < 0.5 {
        return read_i8(m.addr() + (0x1ab + lu)) as u8;
    }
    *factor = h as f32;
    if !(h > 0.3) {
        4
    } else if h < 0.7 {
        3
    } else if h > 1.5 {
        1
    } else {
        2
    }
}

/// The staff object `c` picks the club's formation (`+0x2f`) and loads the tactic into the engine (`0x6e0230`,
/// thiscall, ret 0x10, `match_man.cpp`). Returns 0.
///
/// With the engine at `+0` it draws a random formation; with `modo` != 2 it only reloads; with a human manager to be
/// consulted (`0x5fd210`/`0x5fea50`) it uses his tactic (`0x5fbe20`, `0x5fbd00`).
///
/// Otherwise it rates (`0x6e2f20`) each sector (7, bits 1..0x40) and band (5, bits 0x80..0x800) from the two best of
/// the `n` persons in `fichas` (the goalkeeper only in the middle band; with fewer than 11, the last sector counts half
/// and the wings of sectors 1 to 3 get the lowest), and judges each formation of the tactics bank (`0xde1d08`): the
/// time (`+2`, up to 3000) and the mentality (`0x6df460`) weigh the formation's rating (`+0x10`/`+0x18`/`+0x20`, up to
/// 3), the manager adjusts (`+0x1c`, `+0xe`, `+0x1f`; the favourite `+0x43` counts ×1.1 or up to ×3.75, the others
/// ×0.1) and each slot adds the rating of its sector and band (`0x5b2ae0`) and the marks (`0x5b5270`/`0x5b57c0`); the
/// balance of the sectors comes in through the mentality.
///
/// It saves the best (`0x8967d0`), copies the tactic (`0x896660`) into the engine (`+0x91e2` of the side) and into
/// `+0x2b` and sets the instructions (`0x6ddab0`). The formations' ratings are kept in a list from the game's CRT
/// (`0x945501`/`0x9452ca`). Returns 0.
pub(crate) unsafe extern "thiscall" fn choose_formation(c: usize, sheets: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let c = MatchManager(c);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let pp = exe::match_man_choose_mentality(c) as i8;
    let club = c.club();
    let mut escala = (club.reputation() as i32) << 3;
    if escala <= 2000 {
        escala = 2000;
    }
    let m = c.engine();
    let human = c.tactic_slot() as i32;
    let tat = || read_usize(G_TACTICS + 0);
    let how_many = || read_i32(tat() + tactics_fields::PRESET_TACTIC_COUNT).wrapping_add(read_i32(tat() + tactics_fields::CUSTOM_TACTIC_COUNT));
    let best: i32;
    let tactic = 'tactic: {
        if read_i8(m.addr() + 0) != 0 {
            best = random(how_many()) as i16 as i32;
        } else if a4 as u8 != 2 {
            break 'tactic exe::tactics_get_club_tactic(tat(), club, (human) as u32);
        } else {
            let p = club.manager();
            let q = read_i32(G_STAFF_COUNT);
            if human == 0 && !p.is_null() && *(p.addr() as *const i32) >= q - 0x10 && *(p.addr() as *const i32) < q && (p.classification() as i8) != 0 {
                let manager = read_usize(G_HUMAN_MANAGER + 0);
                if exe::human_manager_is_on_holiday(manager, p) != 0 && exe::human_manager_get_holiday_option(manager, 2, p) != 0 {
                    exe::human_manager_select_tactic_slot(manager, (0) as i32, club, p);
                    break 'tactic exe::human_manager_get_active_tactic(read_usize(G_HUMAN_MANAGER + 0), club.manager().addr());
                }
            }
            best = judge_formations(c, sheets, a2, a3, pp, escala, calloc, free);
        }
        exe::tactics_set_club_tactic_index(tat(), club, best as i8 as i16 as u16 as u32, human);
        exe::tactics_get_club_tactic(tat(), club, (human) as u32)
    };
    let side = c.side() as isize;
    core::ptr::copy_nonoverlapping(tactic as *const u8, (m.addr() as isize + (engine::TACTICS as isize) + (formation_fields::SIZE as isize) * side) as *mut u8, 0x18e3);
    // simplification: without tactics (or without +0x2b) the original stops the game with an error (match_man.cpp 0xc1d); here it just does not copy
    let copy = c.tactics();
    if tactic != 0 && copy != 0 {
        core::ptr::copy_nonoverlapping(tactic as *const u8, copy as *mut u8, 0x18e3);
        exe::match_man_set_computer_instructions(c);
    }
    0
}

/// Core of choose_formation (`exe::match_man_choose_computer_tactic`): rates each sector/band by the best two players and judges each formation;
/// returns the best.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn judge_formations(
    c: MatchManager, sheets: usize, a2: u32, a3: u32, pp: i8, escala: i32, calloc: unsafe extern "C" fn(u32, u32) -> usize, free: unsafe extern "C" fn(usize),
) -> i32 {
    let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let club = c.club();
    let n = a2 as u8 as i8 as i32;
    // the ratings of each sector (g) and band (s): the sum of the two best people
    let mut table = [0i32; 35];
    for g in 0..7 {
        for s in 0..5 {
            let band = if g == 0 {
                if s != 2 {
                    continue;
                }
                0
            } else {
                0x80u32 << s
            };
            let mode = band | 1 << g;
            let (mut a, mut b, mut quantos) = (-10000i32, -10000i32, 0);
            for k in 0..n {
                let p = Staff(read_usize((sheets + 4 * k as usize) + 0));
                let r = rating(p.addr(), p.player().addr(), 0, mode, a3, club.addr(), c.addr(), c.manager() as u32, u32::MAX, 0, 0, 0, 0, 0, 0).max(-10000);
                match quantos {
                    0 => {
                        a = r;
                        quantos = 1;
                    }
                    1 => {
                        if r > a {
                            b = a;
                            a = r;
                        } else {
                            b = r;
                        }
                        quantos = 2;
                    }
                    _ => {
                        if r > a {
                            b = a;
                            a = r;
                        } else if r > b {
                            b = r;
                        }
                    }
                }
            }
            let sum = a.wrapping_add(b);
            table[g * 5 + s] = if n < 11 && g == 6 { sum / 2 } else { sum };
        }
    }
    if n < 11 {
        for (x, y) in [(5, 9), (10, 14), (15, 19)] {
            let v = table[x].min(table[y]);
            table[x] = v;
            table[y] = v;
        }
    }
    let tat = || read_usize(G_TACTICS + 0);
    let how_many = || read_i32(tat() + tactics_fields::PRESET_TACTIC_COUNT).wrapping_add(read_i32(tat() + tactics_fields::CUSTOM_TACTIC_COUNT));
    let list = calloc(how_many() as u32, 8);
    let (mut melhor, mut melhor_r) = (-1i32, 0f64);
    let manager = c.manager_sheet();
    let mut idx = 0i32;
    while idx < how_many() {
        let f = read_usize((tat()) + 0) + 0x28 * idx as usize;
        let mut t = read_i16(f + 2) as i32;
        if t as f64 > 3000.0 {
            t = 3000;
        }
        let v = match pp {
            1 => read_f64(f + tactic_stats::ALL_OUT_ATTACK_SCORE),
            3..=5 => read_f64(f + tactic_stats::DEFENSIVE_SCORE),
            _ => read_f64(f + tactic_stats::ATTACK_SCORE),
        };
        let v = if v > 3.0 { 3.0 } else { v };
        let mut x = ((3000.0 - t as f64) * 5.0 + v * t as f64) * 0.005;
        let tactic = exe::tactics_get_tactic_block(tat(), read_u16(f) as u32, 0);
        let flags = read_u32(tactic + tactics_fields::FLAGS);
        if !manager.is_null() {
            if flags & 0x400 != 0 {
                x += (manager.offside() as i32 - 10) as f64;
            }
            if flags & 0x1000 != 0 {
                x += (10 - manager.attacking() as i32) as f64;
            }
            if flags & 0x100 != 0 {
                x += (manager.pressing() as i32 - 10) as f64;
            }
            let fav = manager.formation();
            let favorita = fav > 0 && fav < 10 && {
                let q = read_usize((tat()) + (8 + 4 * fav as usize));
                q != 0 && q == read_usize(tactic + tactics_fields::FORMATION)
            };
            x *= if !favorita {
                0.1
            } else if n >= 11 {
                let y = n as f64 * 0.2 - 0.2;
                if 3.75 < y { 3.75 } else { y }
            } else {
                1.1
            };
        }
        let (mut f18, mut f20, mut f28, mut f44, mut f54) = (0f32, 0f32, 0f32, 0f32, 0f32);
        let mut sum = 0i32;
        for vaga in 0..11u32 {
            let p1 = exe::formation_get_slot_position(tactic, vaga, 0);
            let p2 = exe::formation_get_slot_position(tactic, vaga, 1) as u8;
            if !manager.is_null() {
                let r = exe::formation_get_player_instructions(tactic, (vaga) as i8) as u32;
                let fl = exe::formation_get_marking_type(tactic, (vaga) as i8);
                if r & 0x8000 != 0 && flags & 0x100 == 0 {
                    x += manager.pressing() as f64 * 0.25 - 2.5;
                }
                if r & 0x200000 != 0 {
                    x += manager.free_roles() as f64 - 10.0;
                }
                if r & 0x800000 != 0 {
                    x += manager.attacking() as f64 * 0.5 - 5.0;
                }
                if fl & 4 != 0 {
                    x += manager.marking() as f64 * 1.5 - 15.0;
                }
            }
            let more = |v: &mut f32, q: f64| *v = (*v as f64 + q) as f32;
            let (mut setor, mut band) = (0usize, -1i32);
            if p1 & 1 != 0 {
                band = 2;
            } else if p1 & 2 != 0 {
                more(&mut f18, 1.0);
                setor = 1;
            } else if p1 & 4 != 0 {
                more(&mut f18, 1.0);
                setor = 2;
            } else if p1 & 8 != 0 {
                more(&mut f28, 1.0);
                setor = 3;
            } else if p1 & 0x10 != 0 {
                more(&mut f54, 1.0);
                setor = 4;
            } else if p1 & 0x20 != 0 {
                more(&mut f20, 1.0);
                setor = 5;
            } else {
                more(&mut f44, 1.0);
                setor = 6;
            }
            if p2 & 2 != 0 {
                more(&mut f18, 0.5);
                setor = 1;
            } else if p2 & 4 != 0 {
                more(&mut f18, 0.5);
                setor = 2;
            } else if p2 & 8 != 0 {
                more(&mut f28, 0.5);
                setor = 3;
            } else if p2 & 0x10 != 0 {
                more(&mut f54, 0.5);
                setor = 4;
            } else if p2 & 0x20 != 0 {
                more(&mut f20, 0.5);
                setor = 5;
            } else if p2 & 0x40 != 0 {
                more(&mut f44, 0.5);
                setor = 6;
            }
            if band == -1 {
                band = if p1 & 0x80 != 0 {
                    0
                } else if p1 & 0x100 != 0 {
                    1
                } else if p1 & 0x200 != 0 {
                    2
                } else if p1 & 0x400 != 0 {
                    3
                } else {
                    4
                };
            }
            sum = sum.wrapping_add(table[setor * 5 + band as usize]);
        }
        // the balance of the sectors
        let z = (f18 as f64 - f44 as f64) as f32 as f64;
        let q = |v: f32| game_pow(v as f64, 2.0);
        let s_ = (q(f44) + q(f44) + q(f20)) - (q(f18) + q(f18)) - q(f28);
        let e = 44.0 - ((32.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;
        let y = match pp {
            1 => (e * 0.015 + 1.0) * x,
            3 => (1.0 - e * 0.015) * x,
            4 => (1.0 - e * 0.025) * x,
            _ => x,
        };
        let r = y * (sum as f64 / escala as f64);
        write_f64(list + 8 * idx as usize, r);
        if melhor == -1 || r > melhor_r {
            melhor_r = r;
            melhor = idx;
        }
        idx += 1;
    }
    if list != 0 {
        free(list);
    }
    melhor
}

/// the staff set the instructions of the tactic copied at +0x2b (`set_formation_flag` general, `set_slot_position` per position, 0x5b5290
/// 64-bit marks, 0x5b5910 man-marking in the engine's tactic block) and in the tactics bank (`extra_tactic_slot`). With a human
/// manager to consult it does nothing. At home (or by a draw against the reputation gap in competitions +0x3a = 1 or 9)
/// it asks for 0x20; the manager (+0x13, +0x1f, +0x1c) picks tempo and marking. A factor (reputations / 17000, manager
/// +0xe and the remainder of the ids' sum by 13 − `exe::non_player_attribute_rating`/2) is compared with the attributes (`staff_get_player_attribute`) of each of the
/// 20 on the side on the pitch: wing runs, man-marking the opponent from `exe::cm3_match_engine_find_marking_target` (with `get_stat` for midfielders),
/// passing, long shots, dribbling, tackling, offside, set pieces (minus the accumulated value of those with high
/// +0x2e), crosses by table and the set-piece taker (`extra_tactic_slot` for whoever is at +6/+0xa/+0xe of the staff). Quirk kept:
/// the marked player's side comes from the player's +0x19 (position 0 becomes the other side)
/// Original: `instructions` (0x6ddab0, match_man.cpp (~6.4 KB), thiscall).
pub(crate) unsafe extern "thiscall" fn instructions(c: usize) {
    let c = MatchManager(c);
    let u = |p: usize, o: usize| read_usize(p + o);
    let w = |p: usize, o: usize| read_i16(p + o);
    let average: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f32 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let (ca, cb) = (u(c.addr(), match_manager::CLUB), u(c.addr(), match_manager::OPPONENT));
    let mut acc = 0i8;
    let f40 = (w(ca, club::REPUTATION) as i32 as f64 + w(cb, club::REPUTATION) as i32 as f64) * 5.882352941176471e-05;
    let human = c.tactic_slot();
    if human == 0 {
        let p = u(ca, club::MANAGER);
        let q = read_i32(G_STAFF_COUNT);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && read_i8(p + staff::CLASSIFICATION) != 0 {
            let manager = u(G_HUMAN_MANAGER, 0);
            if exe::human_manager_is_on_holiday(manager, p) != 0 && exe::human_manager_get_holiday_option(u(G_HUMAN_MANAGER, 0), 2, u(u(c.addr(), match_manager::CLUB), 0xcf)) != 0 {
                return;
            }
        }
    }
    let t = || u(c.addr(), match_manager::TACTICS);
    let in_home = human != 0 || {
        let comp = read_i8(u(c.addr(), match_manager::FIXTURE) + fixture::CATEGORY);
        (comp == 1 || comp == 9) && {
            let r = random(2000).wrapping_add(random(-4000));
            (w(u(c.addr(), match_manager::CLUB), 0x80) as i32 - w(u(c.addr(), match_manager::OPPONENT), 0x80) as i32) > r
        }
    };
    if in_home {
        exe::formation_set_team_instruction(t(), 0x20);
    }
    let tec = NonPlayer(u(c.addr(), match_manager::MANAGER_SHEET));
    if tec.is_null() {
        return;
    }
    let f40 = (1.0 - (tec.attacking() as f64 - 10.0) * 0.03333333333333333) * f40;
    let ritmo = tec.directness();
    let r = if ritmo <= 8 {
        2
    } else if ritmo <= 14 {
        1
    } else if ritmo <= 18 {
        4
    } else if ritmo <= 20 {
        8
    } else {
        0
    };
    if r != 0 {
        exe::formation_set_team_instruction(t(), r);
    }
    let m1f = read_i8(u(c.addr(), match_manager::MANAGER_SHEET) + non_player::PRESSING);
    if m1f >= 15 {
        exe::formation_set_team_instruction(t(), 0x100);
    } else if m1f <= 5 {
        exe::formation_set_team_instruction(t(), 0x80);
    }
    if 19.0 - w(u(c.addr(), match_manager::CLUB), 0x80) as i32 as f64 * 0.0003 < read_i8(u(c.addr(), match_manager::MANAGER_SHEET) + non_player::OFFSIDE) as f64 && *((t() + formation_fields::FORMATION_MASK) as *const u8) & 8 == 0 {
        exe::formation_set_team_instruction(t(), 0x400);
    }
    let tat = || u(G_TACTICS, 0);
    let hum = c.tactic_slot() as i32;
    exe::tactics_set_playmaker(tat(), u(c.addr(), match_manager::CLUB), 0, 0, (hum) as i32 as u32);
    exe::tactics_set_playmaker(tat(), u(c.addr(), match_manager::CLUB), 1, 0, (c.tactic_slot() as i32) as i32 as u32);
    let m = u(c.addr(), match_manager::ENGINE);
    let block = |k: usize| Engine(m).player(c.side(), k).addr();
    // the running total of those who can take set pieces
    for k in 0..20 {
        let j = block(k);
        if read_i8(j + match_player::POSITION) >= 0 && exe::match_player_get_position(j) & 0x40 != 0 && exe::match_player_get_position(j) & 0x200 != 0 {
            let f = u(j, match_player::PLAYER);
            let v = read_i8(f + player_sheet::JUMPING);
            if v > 14 {
                acc = acc.wrapping_add(ftol(game_pow((v as i32 - 14) as f64, 1.5)) as i8);
                let v40 = read_i8(f + player_sheet::TECHNIQUE);
                if v40 < 8 {
                    acc = acc.wrapping_add(8 - v40);
                }
            }
        }
    }
    let tc = |o: usize| read_i8(u(c.addr(), match_manager::MANAGER_SHEET) + o);
    for k in 0..20 {
        let j = block(k);
        if read_i8(j + match_player::POSITION) < 0 {
            continue;
        }
        let f = u(j, match_player::PLAYER);
        let pos_j = read_u8(j + match_player::POSITION) as u32;
        let a = |k: u32| exe::staff_get_player_attribute(u(j, match_player::STAFF), k) as i32;
        let tb = || exe::match_player_get_position(j);
        let fb = |o: usize| read_i8(u(j, match_player::PLAYER) + o);
        let div = 13 - ((exe::non_player_attribute_rating(u(c.addr(), match_manager::MANAGER_SHEET), 0xbu8 as i8) as i8) as i32) / 2;
        let rest = read_i32(u(c.addr(), match_manager::MANAGER_SHEET)).wrapping_add(read_i32(f)) % div;
        let mut f38 = (rest as f64 * 0.05 + 1.0) * f40 * 1.2;
        if f38 < 0.6 {
            f38 = 0.6;
        }
        // avanco pelas pontas
        if read_u32(t() + formation_fields::FORMATION_MASK) & 0x1c00000 == 0 {
            let lat = |band: u16, ref_: usize| tb() & band != 0 && fb(player_sheet::ATTACKING_MIDFIELDER) as i32 >= fb(ref_) as i32 - 1 && fb(player_sheet::CENTRAL) == 20;
            if tb() & 0x10 != 0 && tb() & 0x200 != 0 && fb(player_sheet::ATTACKING_MIDFIELDER) as i32 > fb(player_sheet::MIDFIELDER) as i32 - 1 && fb(player_sheet::CENTRAL) == 20 && f38 * 16.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x220, 0);
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x220, 1);
            } else if tb() & 0x10 != 0 && (exe::match_player_get_secondary_position(j) as u8) & 0x20 == 0 && lat(0x100, 0x13) && f38 * 16.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x220, 1);
            } else if tb() & 0x10 != 0 && (exe::match_player_get_secondary_position(j) as u8) & 0x20 == 0 && lat(0x400, 0x13) && f38 * 16.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x220, 1);
            } else if tb() & 0x40 != 0 && lat(0x200, 0x15) && f38 * 17.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x220, 0);
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x240, 1);
            } else if tb() & 0x40 != 0 && lat(0x100, 0x15) && f38 * 17.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x120, 0);
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x140, 1);
            } else if tb() & 0x40 != 0 && lat(0x400, 0x15) && f38 * 17.0 < a(7) as f64 {
                exe::formation_set_slot_position(t(), pos_j, 0x420, 0);
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x440, 1);
            }
        }
        let fl = |v: f32| read_f32(j + match_player::STAMINA) as f64 > v as f64;
        if (tb() & 2 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0)
            && fl(8250.0)
            && fb(player_sheet::STAMINA) > 12
            && f38 * 17.0 < a(7) as f64
            && fb(player_sheet::FREE_ROLE) > 15
            && f38 * 40.0 < (a(0x14) + (tc(non_player::FREE_ROLES) as i32 - 10) * 3) as f64
        {
            exe::formation_set_player_instruction(t(), pos_j as i8, 0x200000);
            if (read_f32(j + match_player::STAMINA) as f64) < 8500.0 || fb(player_sheet::STAMINA) < 18 || fb(player_sheet::WORK_RATE) > 16 {
                exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x4000);
            }
        } else {
            let b = a(0x14) + a(0xd) + a(7) + (tc(non_player::FREE_ROLES) as i32 + fb(player_sheet::FREE_ROLE) as i32 - 20) * 3;
            if f38 * 40.0 > b as f64 {
                exe::formation_set_player_instruction(t(), pos_j as i8, 0x100000);
            }
        }
        // 0x5b5270 bit 1 e +0x584 bit 8: 2, senao 4
        let dois_ou_quatro = || {
            let v = if exe::formation_get_player_instructions(t(), read_i8(j + match_player::POSITION)) & 1 != 0 && *((t() + formation_fields::FLAGS) as *const u8) & (formation_fields_flags::LONG_BALL as u8) != 0 { 2 } else { 4 };
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), v as u64);
        };
        if tb() & 0x20 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            dois_ou_quatro();
        }
        if (tb() & 8 != 0 || tb() & 0x10 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && f38 * 16.0 > a(7) as f64 {
            dois_ou_quatro();
            let r = exe::cm3_match_engine_find_marking_target(m, (read_u8(j + match_player::SIDE) as u32) as i8, read_i8(j + match_player::POSITION)) as i8;
            if r != -1 {
                let side = (read_i8(j + match_player::POSITION) == 0) as usize;
                let o = Engine(m).player(side as i8, r as usize).addr();
                let blk = || (u(c.addr(), match_manager::ENGINE) as isize + (engine::TACTICS as isize) + (formation_fields::SIZE as isize) * read_i8(j + match_player::SIDE) as isize) as usize;
                let tbo = || exe::match_player_get_position(o);
                let reps = || {
                    let ra = w(u(c.addr(), match_manager::CLUB), 0x80);
                    ra < w(u(c.addr(), match_manager::OPPONENT), 0x80).wrapping_add(0xfa) || w(u(o, match_player::PLAYER), 0xb).wrapping_add(0xfa) > ra
                };
                let sides = || tbo() & 0x200 != 0 || tbo() & 0x100 != 0 || tbo() & 0x400 != 0;
                let resto5 = || read_i32(u(o, match_player::STAFF)) % 5;
                if u(o, match_player::STAFF) != 0 {
                    if (tbo() & 0x20 != 0 || (exe::match_player_get_secondary_position(o) as u8) & 0x20 != 0)
                        && sides()
                        && reps()
                        && a(0x1c) > 12
                        && a(0x11) > 10
                        && a(0x1f) > 10
                        && fb(player_sheet::DEFENSIVE_MIDFIELDER) == 20
                        && tb() & 0x10 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && exe::staff_get_non_player_attribute(u(c.addr(), match_manager::MANAGER), 0xb) as i32 > (resto5() as i8).wrapping_add(10) as i32
                    {
                        if u(u(u(c.addr(), match_manager::ENGINE), 0x4792), 0x1c) != u(c.addr(), match_manager::CLUB) || a(0x1a) > 15 {
                            exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x208, 1);
                        }
                        exe::formation_set_marking_target(blk(), (read_i8(j + match_player::POSITION) as u8 as u32) as u32 as i8, (u(o, match_player::STAFF)) as usize as *const u32);
                    } else if u(o, match_player::STAFF) != 0
                        && tb() & 8 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && (tbo() & 0x20 != 0 || (exe::match_player_get_secondary_position(o) as u8) & 0x20 != 0)
                        && tbo() & 0x200 != 0
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 15
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && exe::staff_get_non_player_attribute(u(c.addr(), match_manager::MANAGER), 0xb) as i32 > (resto5() as i8).wrapping_add(7) as i32
                    {
                        exe::formation_set_marking_target(blk(), (read_i8(j + match_player::POSITION) as u8 as u32) as u32 as i8, (u(o, match_player::STAFF)) as usize as *const u32);
                    } else if u(o, match_player::STAFF) != 0
                        && (tbo() & 0x10 != 0 || (exe::match_player_get_secondary_position(o) as u8) & 0x10 != 0)
                        && sides()
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 12
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && exe::staff_get_non_player_attribute(u(c.addr(), match_manager::MANAGER), 0xb) as i32 > (resto5() as i8).wrapping_add(8) as i32
                    {
                        let fo = u(o, match_player::PLAYER);
                        let g = u(G_PLAYER_STATS, 0);
                        let m1 = average(g, fo, 6, 1, 6.5);
                        let m2 = average(u(G_PLAYER_STATS, 0), u(o, match_player::PLAYER), 6, 2, 6.5);
                        let m3 = average(u(G_PLAYER_STATS, 0), u(o, match_player::PLAYER), 6, 5, 6.5);
                        let (mut st0, mut r3) = (0.0f64, 0.0f32);
                        if m1 as f64 > 2.0 {
                            st0 = m2 as f64 / m1 as f64;
                            r3 = (m3 as f64 / m1 as f64) as f32;
                        }
                        if st0 > 0.35 || r3 as f64 > 0.4 {
                            if (tbo() & 0x20 != 0 || (exe::match_player_get_secondary_position(o) as u8) & 0x20 != 0) && (fb(player_sheet::DEFENSIVE_MIDFIELDER) == 20 || fb(player_sheet::MIDFIELDER) == 20) && a(0x1a) > 14 {
                                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x208, 1);
                            }
                            exe::formation_set_marking_target(blk(), (read_i8(j + match_player::POSITION) as u8 as u32) as u32 as i8, (u(o, match_player::STAFF)) as usize as *const u32);
                        }
                    }
                }
            }
        }
        // the passes
        let q = f38 * 10.0;
        if (a(0x12) as f64) < q {
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x10000000);
        } else if f38 * 13.0 < a(0x12) as f64 {
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x20000000);
        }
        // long shots
        if (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            let a2 = a(2);
            if (tc(non_player::PRESSING) as i32 / 5) as f64 + q > a2 as f64 {
                let a3 = a(0x1f);
                let pj = read_i8(j + match_player::POSITION) as u8 as u32;
                if (tc(non_player::PRESSING) as i32 / 5) as f64 + f38 * 5.0 > a3 as f64 {
                    let v = if exe::formation_get_player_instructions(t(), pj as i8) & 0x200 != 0 && *((t() + formation_fields::FLAGS) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                    exe::formation_set_player_instruction(t(), pj as i8, v as u64);
                    exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x8000);
                } else {
                    let a3 = a(0x1f);
                    if (tc(non_player::PRESSING) as i32 / 5) as f64 + q > a3 as f64 {
                        let v = if exe::formation_get_player_instructions(t(), pj as i8) & 0x200 != 0 && *((t() + formation_fields::FLAGS) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                        exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), v as u64);
                    }
                }
            }
        }
        if read_i8(j + match_player::YELLOW_CARDS) != 0 && a(2) > tc(non_player::PRESSING) as i32 / 4 + 8 {
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x800);
        }
        // dribles
        if tb() & 8 != 0 || tb() & 0x10 != 0 || (tc(non_player::ATTACKING) > 16 && tb() & 0x40 != 0) {
            let f28 = f38 * 14.0;
            let te4 = tc(non_player::ATTACKING) as i32 / 4;
            let pj = || read_i8(j + match_player::POSITION) as u8 as u32;
            let fl8000 = || read_f32(j + match_player::STAMINA) as f64;
            if f28 - (te4 as f64) < a(0x14) as f64 && f28 - ((tc(non_player::ATTACKING) as i32 / 4) as f64) < a(0xc) as f64 && fl8000() > 8000.0 {
                exe::formation_set_player_instruction(t(), pj() as i8, 0x800000);
            } else {
                let te4 = tc(non_player::ATTACKING) as i32 / 4;
                let lim = q - te4 as f64;
                let high = lim > a(0x14) as f64 && lim < a(0xc) as f64;
                if high || fl8000() < 8000.0 {
                    exe::formation_set_player_instruction(t(), pj() as i8, 0x400000);
                }
            }
        }
        // tackles: midfielders and attackers (or good tacklers) go in by the highest rating
        {
            let pj = || read_i8(j + match_player::POSITION) as u8 as u32;
            let enters = tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0 || f38 * 16.0 < a(0xb) as f64;
            let strong = enters && (f38 * 14.0 < a(0xb) as f64 || f38 * 34.0 - ((tc(non_player::ATTACKING) as i32 / 4) as f64) < (a(0x15) + a(0xb)) as f64);
            if strong {
                exe::formation_set_player_instruction(t(), pj() as i8, 0x2000000);
            } else if f38 * 25.0 - (tc(non_player::ATTACKING) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {
                exe::formation_set_player_instruction(t(), pj() as i8, 0x1000000);
            }
        }
        if tb() & 0x10 != 0 && fb(player_sheet::FREE_ROLE) > 12 && fb(player_sheet::ATTACKING_MIDFIELDER) >= fb(player_sheet::MIDFIELDER) {
            if tb() & 0x80 != 0 && fb(player_sheet::CENTRAL) > fb(player_sheet::LEFT_SIDE) {
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x120, 1);
            } else if tb() & 0x800 != 0 && fb(player_sheet::CENTRAL) > fb(player_sheet::RIGHT_SIDE) {
                exe::formation_set_slot_position(t(), read_i8(j + match_player::POSITION) as u8 as u32, 0x420, 1);
            }
        }
        // impedimento
        let d = || a(0x1b) - a(0x15) - a(0x14);
        if tb() & 0x40 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 && d() > 0 {
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x8000000);
        } else if d() < -5 {
            exe::formation_set_player_instruction(t(), read_i8(j + match_player::POSITION), 0x4000000);
        }
        // cobrancas
        let l28 = acc as f64;
        let pj = || read_i8(j + match_player::POSITION) as u8 as u32;
        let (k1, k2) = if tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            (f38 * 16.0, q)
        } else if tb() & 8 == 0 && tb() & 0x10 == 0 && tb() & 0x20 == 0 {
            (f38 * 13.0, f38 * 7.0)
        } else {
            (q, f38 * 5.0)
        };
        if k1 - l28 < a(8) as f64 {
            exe::formation_set_player_instruction(t(), pj() as i8, 4 << 32);
        } else if k2 - l28 > a(8) as f64 {
            exe::formation_set_player_instruction(t(), pj() as i8, 8 << 32);
        }
        // crosses by the table
        let tv = tb() as i16 as i32;
        let b = || a(0x16) + a(7);
        match tv {
            0x210 | 0x110 | 0xa0 | 0x120 | 0x420 | 0x220 | 0x410 | 0x820 => {
                if f38 * 23.0 < b() as f64 {
                    exe::formation_set_player_instruction(t(), pj() as i8, 1 << 32);
                }
            }
            0x90 | 0x108 | 0x202 | 0x208 | 0x408 | 0x810 => {
                if f38 * 27.0 < b() as f64 {
                    exe::formation_set_player_instruction(t(), pj() as i8, 1 << 32);
                }
            }
            _ => {
                if f38 * 31.0 < b() as f64 {
                    exe::formation_set_player_instruction(t(), pj() as i8, 1 << 32);
                } else if f38 * 20.0 > b() as f64 {
                    exe::formation_set_player_instruction(t(), pj() as i8, 2 << 32);
                }
            }
        }
        // set piece
        if (tb() & 0x10 != 0 || tb() & 8 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && (exe::match_player_get_secondary_position(j) as u8) & 0x40 == 0 {
            let ca = u(c.addr(), match_manager::CLUB);
            let s = a(0x16) + a(9) + a(7);
            if ((w(ca, club::REPUTATION) as i32 / 500) as f64 + f38 * 30.0) < s as f64 {
                let pj = u(j, match_player::STAFF);
                if u(c.addr(), 6) == pj || u(c.addr(), match_manager::KEY_PLAYER_2) == pj || u(c.addr(), match_manager::KEY_PLAYER_3) == pj {
                    exe::tactics_set_playmaker(tat(), ca, 0, pj, (c.tactic_slot() as i32) as i32 as u32);
                }
            }
        }
    }
}

/// Port of `exe::match_man_place_player_in_team` (thiscall, ret 0xc, match_man.cpp): puts `pessoa` in slot `pos` (0..19) of the staff block (+0x23)
/// via `exe::match_player_setup` (engine +0x12, side +0x18, `extra`). Without a person returns 0; otherwise 1.
///
/// simplification: a slot outside 0..19 stops the game with an error in the original (match_man.cpp 0xc39); here it
/// just returns 0.
///
/// `exe::match_player_setup` (initialises the player, with random draws and effects outside the engine) is injected: in the comparison
/// it is a tape unit (the original records; the Rust copy checks and does not run it again).
pub(crate) unsafe fn place_in_slot_with(c: MatchManager, pos: u32, person: usize, extra: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let p = pos as u8 as i8;
    if person == 0 || !(0..=19).contains(&p) {
        return 0;
    }
    let j = c.player(p as usize).addr();
    call(FN_MATCH_PLAYER_SETUP, j, [c.engine().addr() as u32, person as u32, (c.side() as u8) as u32, pos, extra]);
    1
}

/// Hook entry for `exe::match_man_place_player_in_team` (place_in_slot_with): puts `pessoa` in slot `pos` (0..19) of the staff's player block
/// (+0x23), setting up the match player (`exe::match_player_setup`) with the engine, side and `extra`; returns 1, or 0 without a person
/// or with a bad slot.
pub(crate) unsafe extern "thiscall" fn put_in_place(c: usize, pos: u32, person: usize, extra: u32) -> u8 {
    place_in_slot_with(MatchManager(c), pos, person, extra, &mut |_, this, a| {
        exe::match_player_setup(this, a[0], a[1], a[2], a[3], a[4]);
        0
    })
}

// Port of 0x6eaea0 (starts the player in the slot): a tape unit in comparison.
pub(crate) static INIT_MATCH_PLAYER_TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
pub(crate) const PROLOGUE_6EAEA0: Prologue = Prologue { address: 0x6eaea0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c] };

/// Hook entry for `exe::match_player_setup` (initialise match player): runs the original directly, or records it as a tape unit while
/// recording.
pub(crate) unsafe extern "thiscall" fn init_player_hook(j: usize, m: u32, person: u32, side: u32, pos: u32, extra: u32) {
    let o: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(INIT_MATCH_PLAYER_TRAMPOLINE.load(Relaxed));
    if !is_recording() {
        return o(j, m, person, side, pos, extra);
    }
    call_recording(FN_MATCH_PLAYER_SETUP, j, [m, person, side, pos, extra], || {
        o(j, m, person, side, pos, extra);
        0
    });
}

/// Comparator ends for the substitution sort: players without person go last, +0x19 = -2 and goalkeepers with +0xf < 20
/// first; None when the rest decides.
pub(crate) unsafe fn substitution_order_ends(a: MatchPlayer, b: MatchPlayer) -> Option<i32> {
    if a.is_null() || a.player().is_null() {
        return Some(1);
    }
    if b.is_null() || b.player().is_null() {
        return Some(-1);
    }
    if a.position() == match_slot::INJURED_OFF {
        return Some(-1);
    }
    if b.position() == match_slot::INJURED_OFF {
        return Some(1);
    }
    if exe::match_player_get_position(a) == 1 && a.player().goalkeeper() < 20 {
        return Some(-1);
    }
    if exe::match_player_get_position(b) == 1 && b.player().goalkeeper() < 20 {
        return Some(1);
    }
    None
}

/// Player's condition: +0x35 (i16) + +0x4d (f32), truncated.
pub(crate) unsafe fn match_condition(j: MatchPlayer) -> i32 {
    ftol(j.match_rating() as f64 + j.stamina() as f64)
}

/// qsort comparator for substitutions: compares two match players by condition (+0x35 + +0x4d), weighted by the first
/// player's position (x1.35 for 2 and 4, x1.2/x1.25 for 8, x1.1 for 0x10). Returns a - b.
pub(crate) unsafe extern "C" fn order_by_table(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = substitution_order_ends(MatchPlayer(a), MatchPlayer(b)) {
        return r;
    }
    let pesar = |v: i32, oito: f64| -> i32 {
        match exe::match_player_get_position(a) {
            4 | 2 => ftol(v as f64 * 1.35),
            8 => ftol(v as f64 * oito),
            0x10 => ftol(v as f64 * 1.1),
            _ => v,
        }
    };
    let va = pesar(match_condition(MatchPlayer(a)), 1.2);
    let vb = pesar(match_condition(MatchPlayer(b)), 1.25);
    va.wrapping_sub(vb)
}

/// qsort comparator for substitutions: after the fixed ends (players without person last, goalkeepers first...), orders
/// by match condition (+0x35 + +0x4d), ascending.
pub(crate) unsafe extern "C" fn order_by_condition(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = substitution_order_ends(MatchPlayer(a), MatchPlayer(b)) {
        return r;
    }
    let vb = match_condition(MatchPlayer(b));
    match_condition(MatchPlayer(a)).wrapping_sub(vb)
}

/// for each place from 0 to 10 + (match +0x4d & 0xf), the person from the club's tactic (`tactic_slot_person`) comes in (`exe::match_man_place_player_in_team`)
/// if able to play (`get_player_injury_status` ≠ 2, `is_player_suspended` and `get_international_duty_club` without impediment); returns how many
/// Original: `starters` (0x6dbb80, match_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn starters(c: usize) -> u8 {
    starters_with(MatchManager(c), &mut |_, this, a| {
        exe::match_man_place_player_in_team(this, a[0], a[1] as usize, a[2]) as u32
    })
}

/// Port of `starters` with `exe::match_man_place_player_in_team` injected: places each tactic-selected player who is not injured, banned or on
/// international duty; returns how many.
pub(crate) unsafe fn starters_with(c: MatchManager, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let total = || ((c.fixture().squad_rules() as u8) & 0xf) as i32 + 11;
    let mut n = 0u8;
    let mut k = 0i8;
    while (k as i32) < total() {
        let pos = k as u8 as u32;
        let p = exe::tactics_get_selected_player(read_usize(G_TACTICS + 0), c.club(), pos, c.tactic_slot() as i32);
        if p != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY + 0), p) != 2 {
            let game = c.fixture();
            let free = read_i32(game.addr()) == -2 || exe::discipline_is_player_banned(read_usize(G_DISCIPLINE + 0), p, game.competition(), 1, 0) == 0;
            if free && exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS + 0), p, G_CURRENT_DATE, c.club(), 0, 0) == 0 && call(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c.addr(), [pos, p as u32, pos, 0, 0]) as u8 != 0 {
                n = n.wrapping_add(1);
            }
        }
        k = k.wrapping_add(1);
    }
    n
}

/// stores the staff's tactic (+0x3f) in the bank (`write_tactics_row`); with a human manager only when `exe::human_manager_is_human_club` denies or `exe::human_manager_controls_reserve_team`
/// confirms
/// Original: `store_tactic` (0x6dbb10, match_man.cpp, thiscall, virtual).
pub(crate) unsafe extern "thiscall" fn store_tactic(c: usize) {
    let c = MatchManager(c);
    let u = |p: usize, o: usize| read_usize(p + o);
    let club = u(c.addr(), match_manager::CLUB);
    if club == 0 || u(G_TACTICS, 0) == 0 {
        return;
    }
    if c.tactic_slot() != 0 {
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && exe::human_manager_controls_reserve_team(u(G_HUMAN_MANAGER, 0), u(u(c.addr(), match_manager::CLUB), 0xcf)) == 0 {
            return;
        }
    }
    exe::tactics_store_selection_record(u(G_TACTICS, 0), u(c.addr(), match_manager::CLUB), c.addr() + match_manager::TACTICS_ROW, c.tactic_slot() as i32);
}

/// Shows the game's "v%s %s.%s %d" error box for source file `arquivo` and line (game continues), clearing
/// G_MOUSE_LEFT_BUTTON_DOWN.
pub(crate) unsafe fn show_error_at(file: usize, row: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(file, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// the staff selects the team against the opponent. Outside the human, weighs today's match against the next one on the
/// list (`find_fixtures`: competition reputation, rival, same city, days until it) to rest players (+0x1a 0/1/2 and the slot
/// masks); filters the 50 of the lines (state, suspension, already played, barred, on loan), selects (0x6e2000), picks
/// the formation (`exe::match_man_choose_computer_tactic`), the three references (+6/+0xa/+0xe), the rotation (`transfer_manager_check_matchday_squad_limit`/`apply_squad_rule`), the slots
/// (`exe::match_man_place_player_in_team`), the captain (0x897760), the instructions (`instructions`) and the general orders by the stance (`set_formation_flag`). The
/// locals live in a frame with the original stack offsets (zeroed, like the determinised original)
/// Original: `select_automatically` (0x6dbc50, match_man.cpp (~7.7 KB), thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn select_automatically(t: usize, other: usize, mode: u32) -> i8 {
    let t = MatchManager(t);
    let mut quadro = [0u8; 0x750];
    let f = quadro.as_mut_ptr() as usize;
    let _b = |o: usize| (f + o) as *mut u8;
    let rd = |o: usize| core::ptr::read_unaligned((f + o) as *const u32);
    let wd = |o: usize, v: u32| core::ptr::write_unaligned((f + o) as *mut u32, v);
    let rw = |o: usize| core::ptr::read_unaligned((f + o) as *const i16);
    let ww = |o: usize, v: i16| core::ptr::write_unaligned((f + o) as *mut i16, v);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let h16 = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);

    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let rodizio: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    #[allow(clippy::type_complexity)]
    let capitao: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);

    let club = u(t.addr(), match_manager::CLUB);
    let game = Fixture(u(t.addr(), match_manager::FIXTURE));
    let m = u(t.addr(), match_manager::ENGINE);
    let side = t.side() as isize;
    let weight_of_side = || *(((m as isize) + (engine::SIDE_PITCH_HANDICAP as isize) + side) as *const i8);
    let competition = |id: i32| *((u(G_COMP_OBJECT_TABLE, 0) as isize + 4 * id as isize) as *const usize);
    let tactic = *(G_TACTICS as *const usize);
    let manager = *(G_HUMAN_MANAGER as *const usize);
    let mut mode = mode;

    exe::new_llist(f + 0x80);
    exe::date_date_54a110(f + 0x38);
    exe::date_add_days(G_CURRENT_DATE, f + 0xac, 2);
    if other == 0 {
        match_man_fatal_error(0x174);
    }
    *((f + 0x13) as *mut u8) = (*((u(m, engine::MATCH) + fixture::SQUAD_RULES) as *const u8) & 0xf).wrapping_add(0xb);
    let strong = exe::database_count_club_squad(club, 0, 1) as i8;
    *((f + 0x12) as *mut u8) = strong as u8;
    t.set_selection_flag((0) as i8);

    // outside the human: weighs today’s match against the next one in the list (save players for the more important one?)
    if t.tactic_slot() == 0 && h16(club, club::REPUTATION) >= 0x1770 && *(game.addr() as *const i32) != -2 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::database_is_national_team(club) == 0 && strong > 0x1e {
        exe::date_add_days(G_CURRENT_DATE, f + 0xa4, 1);
        exe::date_add_days(G_CURRENT_DATE, f + 0x40, 7);
        let r = exe::fixture_manager_find_fixtures(u(G_FIXTURE_MANAGER, 0), rd(0xa4), rd(0xa6), rd(0x40), rd(0x42), *(club as *const u32), f + 0x80, u32::MAX, 0xffff_fffe, 1) as u32;
        if r as u16 != 0 {
            // 333 x the competition’s reputation (.69; that of [0x9cf6f8] in table [0xae23d0]) + reputation/3
            let weight = |comp: usize, rival: usize| -> i32 {
                let c = if *(comp as *const u32) == *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32) { u(G_CLUB_COMP_TABLE, 0) + u(KEY_CLUB_COMP_UEFA_CUP, 0).wrapping_mul(0x6b) } else { comp };
                (h16(c, 0x69) as i32 * 333).wrapping_add(h16(rival, club::REPUTATION) as i32 / 3)
            };
            let comp = u(game.addr(), fixture::COMPETITION);
            let id = *(comp as *const u32);
            let mut l14 = weight(comp, other);
            if id != *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32) {
                if (game.category() as i8) == 8 {
                    let k = game.round_name() as u16;
                    if t.side() == 1 || (0x78..=0x7c).contains(&k) || (0x82..=0x87).contains(&k) || (0x96..=0x9b).contains(&k) {
                        t.set_selection_flag((1) as i8);
                    }
                } else if id == *(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as *const u32) && (game.stage_index() as i8) == -1 {
                    t.set_selection_flag((1) as i8);
                }
            }
            let mut factor = weight_of_side() as f64 * read_f64(rdata::F64_0_06666666667) + read_f64(rdata::F64_1);
            if factor > read_f64(rdata::F64_1_4) {
                factor = read_f64(rdata::F64_1_4);
            }
            let mut esi = ftol(l14 as f64 * factor);
            l14 = esi;
            if exe::comp_util_get_league_clash_type(other, club) != 0 {
                esi = ftol(l14 as f64 * read_f64(rdata::F64_1_75));
                l14 = esi;
                t.set_selection_flag((1) as i8);
            } else if other == u(club, club::RIVAL1) || other == u(club, club::RIVAL2) || other == u(club, club::RIVAL3) {
                esi = ftol(l14 as f64 * read_f64(rdata::F64_1_5));
                l14 = esi;
            }
            if exe::comp_is_club_still_in_contention(competition(*(game.addr() as *const i32)), club) == 0 {
                t.set_selection_flag((0) as i8);
                let tec = Staff(u(t.addr(), match_manager::MANAGER));
                l14 = if !tec.is_null() { ftol(tec.sportsmanship() as f64 * read_f64(rdata::F64_0_04) * l14 as f64) } else { esi / 2 };
            }
            wd(0x14, l14 as u32);
            wd(0x8a, rd(0x82));
            let x = exe::llist_get_next(f + 0x80);
            wd(0x20, x as u32);
            if x != 0 && *(x as *const i32) != -2 {
                let d = exe::new_date(f + 0x24, *((x + fixture::DAY) as *const u16) as u32, *((x + fixture::YEAR) as *const u16) as u32);
                exe::date_operator_3d(f + 0x38, f + 0x18, d);
                let rival = if club == u(x, 0x1c) { u(x, 0x20) } else { u(x, 0x1c) };
                let mut l18 = weight(u(x, 0x14), rival);
                let mut esi = l18;
                if exe::comp_util_get_league_clash_type(rival, club) != 0 {
                    esi = ftol(l18 as f64 * read_f64(rdata::F64_1_5));
                    l18 = esi;
                } else if rival == u(club, club::RIVAL1) || rival == u(club, club::RIVAL2) || rival == u(club, club::RIVAL3) {
                    esi = ftol(l18 as f64 * read_f64(rdata::F64_1_35));
                    l18 = esi;
                }
                if u(x, 0x18) != u(club, club::STADIUM) {
                    let c = if u(club, club::NATION) != u(rival, club::NATION) { read_f64(rdata::F64_1_2) } else { read_f64(rdata::F64_1_1) };
                    esi = ftol(l18 as f64 * c);
                    l18 = esi;
                }
                if exe::comp_is_club_still_in_contention(competition(*(x as *const i32)), club) == 0 {
                    l18 = esi / 3;
                } else if exe::comp_util_is_league_division(*(game.addr() as *const i32)) != 0 && (read_i8(x + fixture::CATEGORY) == 8 || read_i8(x + fixture::CATEGORY) == 7) && *(x as *const i32) > -1 {
                    let c = competition(*(x as *const i32));
                    if c != 0 && u(c, 8) != 0 {
                        let r = exe::comp_stats_find_club_entry(u(c, 8), club);
                        let k = if r != 0 { *((r + comp_stats_club::FINAL_POSITION) as *const u8) } else { 0 };
                        if k == 0 {
                            l18 = esi / 2;
                        } else if k > 6 {
                            l18 = esi.wrapping_mul(6) / k as i32;
                        }
                    }
                }
                wd(0x18, l18 as u32);
                // the next match in 1..7 days weighs more (the one in 4 days, nothing)
                for (k, c) in [Some(0x96d4c0usize), Some(0x96d4b8), Some(0x96d4b0), None, Some(0x96d4a8), Some(0x96d4a0), Some(0x96d498)].iter().enumerate() {
                    let p = exe::date_add_days(G_CURRENT_DATE, f + 0x48, k as u32 + 1);
                    wd(0x24, *(p as *const u32));
                    wd(0x28, *((p + 4) as *const u32));
                    if rw(0x26) == rw(0x3a) && rw(0x24) == rw(0x38) {
                        if let Some(c) = c {
                            l18 = ftol(l18 as f64 * read_f64(*c));
                            wd(0x18, l18 as u32);
                        }
                        break;
                    }
                }
                let (a, b2) = (l18 as f64, l14 as f64);
                if a > b2 + b2 && strong > 0x28 && l14 < 0x1b58 {
                    t.set_selection_mode((rotation_mode::ROTATE_HEAVY) as i8);
                } else if a > b2 * read_f64(rdata::F64_1_4) && strong > 0x23 && l14 < 0x2134 {
                    t.set_selection_mode((rotation_mode::ROTATE_MEDIUM) as i8);
                } else if a > b2 * read_f64(rdata::F64_1_1) && l14 < 0x2710 {
                    t.set_selection_mode((rotation_mode::ROTATE_LIGHT) as i8);
                }
            }
        }
    }

    // resting players: the slot masks and the first scale, with those who can play
    let loaned_away = |p: usize| -> bool {
        // loaned (0x4cf6d0) to a club other than the partner (0x540a50)?
        let owner = |p: usize| {
            let c = exe::contract_manager_get_contract(u(G_CONTRACT_MANAGER, 0), p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 { u(G_CLUB_TABLE, 0).wrapping_add(u(c, 4).wrapping_mul(club::SIZE)) } else { 0 }
        };
        if (exe::contract_manager_get_loan_contract(u(G_CONTRACT_MANAGER, 0), p) as u32) == 0 {
            return false;
        }
        if owner(p) == club {
            return true;
        }
        owner(p) == partner(club, 0, 1)
    };
    if t.selection_mode() != rotation_mode::NO_ROTATION && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::database_is_national_team(club) == 0 {
        match t.selection_mode() {
            0 => {
                for (o, v) in [(0x50, 1), (0x52, 0x804), (0x54, 0x84), (0x56, 0x204), (0x58, 0x204), (0x5a, 0x204), (0x5c, 0x810), (0x5e, 0x90), (0x60, 0x210), (0x62, 0x210), (0x64, 0x210), (0x66, 0x40), (0x68, 0x40), (0x6a, 0x40), (0x6c, 0x40)] {
                    ww(o, v);
                }
                *((f + 0x30) as *mut u8) = 0xe;
            }
            1 => {
                for (o, v) in [(0x50, 4), (0x52, 4), (0x54, 0x204), (0x56, 0x10), (0x58, 0x10), (0x5a, 0x210), (0x5c, 0x40), (0x5e, 0x40)] {
                    ww(o, v);
                }
                *((f + 0x30) as *mut u8) = 7;
            }
            2 => {
                for (o, v) in [(0x50, 4), (0x52, 0x10), (0x54, 0x40)] {
                    ww(o, v);
                }
                *((f + 0x30) as *mut u8) = 2;
            }
            _ => {}
        }
        let x = rd(0x20) as usize;
        *((f + 0x18) as *mut u8) = 0;
        for k in 0..50usize {
            let p = u(u(t.addr(), match_manager::SQUAD_ROWS), 4 * k);
            if p == 0 || u(p, 0x61) == 0 {
                continue;
            }
            if read_i8(p + staff::AGE) < 0x10 {
                let c = u(game.addr(), fixture::COMPETITION);
                if c != 0 && h16(c, 0x69) > 10 {
                    continue;
                }
            }
            if exe::injury_get_player_injury_status(u(G_INJURY, 0), p) == 2 {
                continue;
            }
            if t.tactic_slot() != 0 {
                if *(game.addr() as *const i32) != -2 && exe::discipline_is_player_banned(u(G_DISCIPLINE, 0), p, u(game.addr(), fixture::COMPETITION), 1, 0) != 0 {
                    continue;
                }
            } else if x != 0 {
                if u(game.addr(), fixture::COMPETITION) != u(x, 0x14) && exe::discipline_is_player_banned(u(G_DISCIPLINE, 0), p, u(x, 0x14), 1, 0) != 0 {
                    continue;
                }
                if exe::transfer_manager_get_ineligibility_code(u(G_TRANSFER_MANAGER, 0), p, x) != 0 || exe::nationalteams_get_international_duty_club(u(G_NATIONAL_TEAMS, 0), p, f + 0x38, club, 0, 0) != 0 {
                    continue;
                }
            }
            let day = if *(game.addr() as *const u32) == *(KEY_CLUB_COMP_RESERVE as *const u32) { f + 0xac } else { G_CURRENT_DATE };
            if exe::nationalteams_get_international_duty_club(u(G_NATIONAL_TEAMS, 0), p, day, club, 0, 0) != 0 || loaned_away(p) {
                continue;
            }
            let c = *((f + 0x18) as *mut u8) as usize;
            wd(0x278 + 4 * c, p as u32);
            *((f + 0x18) as *mut u8) = (c + 1) as u8;
        }
        if *((f + 0x18) as *mut u8) != 0 {
            let r = exe::match_man_choose_captain(t, f + 0x278, rd(0x18));
            t.set_best_player(r);
            if r == 0 {
                match_man_fatal_error(0x302);
            }
        }
        let cf = u(club, club::MANAGER);
        let a7 = if t.tactic_slot() == 0 {
            0xa520c2
        } else if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && cf != 0 && exe::human_manager_is_on_holiday(manager, cf) != 0 && exe::human_manager_get_holiday_option(manager, 3, u(club, club::MANAGER)) != 0 {
            0xc520c2
        } else {
            0x8520c2
        };
        pick(f + 0x278, rd(0x18), f + 0x50, rd(0x30), 0, f + 0x56c, a7, club, t.addr(), u(t.addr(), match_manager::MANAGER) as u32, 0, 0, 0, 0, 0);
    }

    // the candidates: whoever can play enters the list; in mode 1 those of the marked slots only fill them
    let humano19 = t.tactic_slot() as i32 as u32;
    let bl = mode as u8;
    if bl == 2 {
        exe::tactics_clear_or_restore_selection(tactic, club, humano19, 0, 0);
    }
    let slot = |p: usize| exe::tactics_find_selection_slot(tactic, club, p, humano19) as i8;
    let mark_se = |p: usize| {
        if bl == 1 && slot(p) > -1 {
            exe::tactics_remove_player_from_selection(tactic, club, p, humano19, 1);
        }
    };
    *((f + 0x18) as *mut u8) = 0;
    for k in 0..50usize {
        wd(0x20, 4 * k as u32);
        let p = u(u(t.addr(), match_manager::SQUAD_ROWS), 4 * k);
        if p == 0 || u(p, 0x61) == 0 {
            continue;
        }
        if t.tactic_slot() == 0 && read_i8(p + staff::AGE) < 0x10 {
            let c = u(game.addr(), fixture::COMPETITION);
            if c != 0 && h16(c, 0x69) > 10 {
                continue;
            }
        }
        let m5e = |p: usize| *((p + staff::PLAYING_SQUAD) as *const u8);
        let follows = if t.selection_mode() == rotation_mode::NO_ROTATION {
            !(exe::database_is_national_team(club) != 0 && m5e(p) & playing_squad::NATIONAL_B_SQUAD != 0)
        } else if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
            if exe::database_is_national_team(club) != 0 {
                m5e(p) & playing_squad::NATIONAL_B_SQUAD != 0
            } else if t.tactic_slot() != 0 && m5e(p) & playing_squad::FIRST_TEAM != 0 && slot(p) == -1 {
                false
            } else {
                t.tactic_slot() != 0 || m5e(p) & playing_squad::RESERVES == 0
            }
        } else if *(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            true
        } else if exe::database_is_national_team(club) != 0 {
            m5e(p) & playing_squad::NATIONAL_SQUAD == 0
        } else {
            let mut away = false;
            let mut j = 0i8;
            while j < *((f + 0x30) as *mut u8) as i8 {
                if rd(0x56c + 4 * j as usize) as usize == p {
                    away = true;
                    break;
                }
                j += 1;
            }
            !away
        };
        if !follows {
            continue;
        }
        if exe::database_is_national_team(club) == 0 && exe::transfer_manager_player_needs_work_permit(u(G_TRANSFER_MANAGER, 0), p, club, 0) != 0 {
            mark_se(p);
            continue;
        }
        if exe::injury_get_player_injury_status(u(G_INJURY, 0), p) == 2 || (t.tactic_slot() != 0 && exe::injury_get_player_injury_status(u(G_INJURY, 0), p) != 0) {
            mark_se(p);
            continue;
        }
        let away = (*(game.addr() as *const i32) != -2 && exe::discipline_is_player_banned(u(G_DISCIPLINE, 0), p, u(game.addr(), fixture::COMPETITION), 1, 0) != 0)
            || exe::nationalteams_get_international_duty_club(u(G_NATIONAL_TEAMS, 0), p, G_CURRENT_DATE, club, 0, 0) != 0
            || (*(game.addr() as *const i32) != -2 && exe::transfer_manager_get_ineligibility_code(u(G_TRANSFER_MANAGER, 0), p, game) != 0)
            || loaned_away(p);
        if away {
            mark_se(p);
            continue;
        }
        if bl == 1 {
            let v = slot(p);
            if v > -1 && v < *((f + 0x13) as *mut u8) as i8 {
                *((f + 0x2e) as *mut u8) = 1;
                *((f + 0x90 + v as usize) as *mut u8) = 1;
                continue;
            }
        }
        let c = *((f + 0x18) as *mut u8) as usize;
        wd(0x278 + 4 * c, p as u32);
        *((f + 0x18) as *mut u8) = (c + 1) as u8;
    }
    wd(0x20, 0xc8);
    let mut bl = bl;
    if bl == 1 && *((f + 0x2e) as *mut u8) == 0 {
        bl = 2;
        mode = (mode & !0xff) | 2;
    }

    // the scale marks
    let mut mask: u32 = if exe::database_is_national_team(club) != 0 { 0x208a09c } else { 0xca0ca };
    if t.tactic_slot() != 0 {
        mask = (mask | 0x20100) ^ 0x80000;
    } else {
        let dif = || h16(club, club::REPUTATION) as i32 - h16(other, club::REPUTATION) as i32;
        let a = game.category() as i8;
        let mut done = false;
        if a == 1 || a == 9 {
            let lim = exe::utils_random(2000).wrapping_add(exe::utils_random(-4000));
            if dif() > lim {
                mask |= 0x100;
                done = true;
            }
        }
        if !done {
            let lim = exe::utils_random(3000) + 3000;
            if dif() > lim {
                mask |= 0x100;
                if exe::database_is_national_team(club) == 0 {
                    mask |= 0x10000;
                }
            } else {
                let j0 = *(game.addr() as *const i32);
                if t.selection_mode() != rotation_mode::NO_ROTATION || (j0 > -1 && competition(j0) != 0 && exe::comp_is_club_still_in_contention(competition(j0), club) == 0) {
                    mask = (mask | 0x100) ^ 0x80000;
                } else if exe::database_is_national_team(club) == 0 {
                    mask |= 0x10000;
                }
            }
        }
    }
    if t.selection_flag() != 0 {
        mask |= 0x800000;
    }
    wd(0x14, mask);

    // the three references (the highest record +0xd + +0xb, without repeating)
    if *((f + 0x18) as *mut u8) != 0 && bl == 2 {
        let r = exe::match_man_choose_captain(t, f + 0x278, rd(0x18));
        t.set_best_player(r);
        if r == 0 {
            match_man_fatal_error(0x42b);
        }
        if mask & 0x80000 != 0 {
            for s in 0..3usize {
                ww(0x40 + 2 * s, 0xd8f0u16 as i16);
                ww(0x24 + 2 * s, -1);
                let mut c = 0i8;
                while c < *((f + 0x18) as *mut u8) as i8 {
                    let p = rd(0x278 + 4 * c as usize) as usize;
                    let c16 = c as i16;
                    let skips = p == 0
                        || *(p as *const i32) >= *(G_STAFF_COUNT as *const i32)
                        || u(p, 0x61) == 0
                        || (s == 1 && rw(0x24) == c16)
                        || (s == 2 && (rw(0x24) == c16 || rw(0x26) == c16));
                    if !skips {
                        let fi = u(p, 0x61);
                        let v = h16(fi, player_sheet::WORLD_REPUTATION).wrapping_add(h16(fi, player_sheet::CURRENT_REPUTATION));
                        if v > rw(0x40 + 2 * s) {
                            ww(0x24 + 2 * s, c16);
                            ww(0x40 + 2 * s, v);
                        }
                    }
                    c += 1;
                }
                let a = rw(0x24 + 2 * s);
                if a > -1 {
                    core::ptr::write_unaligned((t.addr() + match_manager::KEY_PLAYER_1 + 4 * s) as *mut u32, rd(0x278 + 4 * a as usize));
                }
            }
        }
    }
    exe::match_man_choose_computer_tactic(t, f + 0x278, rd(0x18), rd(0x14), mode);

    // the slots: the 11 of the formation (squares, marks) and the extra drawn ones
    let n = *((f + 0x13) as *mut u8) as i8 as i32;
    wd(0x30, n as u32);
    *((f + 0x12) as *mut u8) = 0;
    *((f + 0x20) as *mut u8) = 0;
    *((f + 0x40) as *mut u8) = 0;
    let tat = || u(t.addr(), match_manager::TACTICS);
    let extra = |o: usize, k: usize, first: i32| {
        if k == 12 && *((f + 0x2f) as *mut u8) == 0 && (*((f + 0x13) as *mut u8) as i8 > 14 || exe::utils_random(first) == 0) {
            *((f + 0x2f) as *mut u8) = 1;
            true
        } else {
            let v = rw(o);
            ww(o, if v < 3 { 4 } else if v >= 6 { 0x40 } else { 0x10 });
            false
        }
    };
    for k in 0..n.max(0) as usize {
        if mode as u8 == 1 {
            if *((f + 0x90 + k) as *mut u8) != 0 {
                continue;
            }
            let c = *((f + 0x12) as *mut u8) as i8 as usize;
            if k <= 10 {
                ww(0xe8 + 2 * c, exe::formation_get_slot_position(tat(), k as u32, 0) as i16);
                ww(0x214 + 2 * c, exe::formation_get_slot_position(tat(), k as u32, 1) as i16);
                let q = exe::formation_get_player_instructions(tat(), (k as u32) as i8);
                wd(0x5bc + 8 * c, q as u32);
                wd(0x5c0 + 8 * c, (q >> 32) as u32);
                *((f + 0xb4 + c) as *mut u8) = exe::formation_get_marking_type(tat(), (k as u32) as i8);
                *((f + 0x20) as *mut u8) = (*((f + 0x20) as *mut u8)).wrapping_add(1);
            } else {
                wd(0x5bc + 8 * c, 0);
                wd(0x5c0 + 8 * c, 0);
                *((f + 0xb4 + c) as *mut u8) = 0;
                ww(0xe8 + 2 * c, exe::utils_random(10) as i16);
                ww(0x214 + 2 * c, 0);
                if extra(0xe8 + 2 * c, k, 0x14) {
                    ww(0xe8 + 2 * c, 1);
                }
                *((f + 0x40) as *mut u8) = (*((f + 0x40) as *mut u8)).wrapping_add(1);
            }
            *((f + 0x12) as *mut u8) = (*((f + 0x12) as *mut u8)).wrapping_add(1);
        } else if k <= 10 {
            *((f + 0x12) as *mut u8) = (*((f + 0x12) as *mut u8)).wrapping_add(1);
            *((f + 0x20) as *mut u8) = (*((f + 0x20) as *mut u8)).wrapping_add(1);
            ww(0xe8 + 2 * k, exe::formation_get_slot_position(tat(), k as u32, 0) as i16);
            ww(0x214 + 2 * k, exe::formation_get_slot_position(tat(), k as u32, 1) as i16);
            let q = exe::formation_get_player_instructions(tat(), (k as u32) as i8);
            wd(0x5bc + 8 * k, q as u32);
            wd(0x5c0 + 8 * k, (q >> 32) as u32);
            *((f + 0xb4 + k) as *mut u8) = exe::formation_get_marking_type(tat(), (k as u32) as i8);
        } else {
            *((f + 0x40) as *mut u8) = (*((f + 0x40) as *mut u8)).wrapping_add(1);
            *((f + 0x12) as *mut u8) = (*((f + 0x12) as *mut u8)).wrapping_add(1);
            wd(0x5bc + 8 * k, 0);
            wd(0x5c0 + 8 * k, 0);
            *((f + 0xb4 + k) as *mut u8) = 0;
            ww(0xe8 + 2 * k, exe::utils_random(10) as i16);
            // as in the original: zeroes the short of the next slot (this one keeps what it had)
            ww(0x214 + 2 * (*((f + 0x12) as *mut u8) as i8 as usize), 0);
            if extra(0xe8 + 2 * k, k, 10) {
                ww(0x100, 1);
            }
        }
    }
    let cf = u(club, club::MANAGER);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && t.tactic_slot() == 0 && cf != 0 && exe::human_manager_is_on_holiday(manager, cf) != 0 && exe::human_manager_get_holiday_option(manager, 3, u(club, club::MANAGER)) != 0 {
        wd(0x14, rd(0x14) | 0x400000);
    }
    pick(f + 0x278, rd(0x18), f + 0xe8, rd(0x20), rd(0x40), f + 0x14c, rd(0x14), club, t.addr(), u(t.addr(), match_manager::MANAGER) as u32, 0, f + 0x5bc, f + 0xb4, *((u(t.addr(), match_manager::TACTICS) + formation_fields::FLAGS) as *const u32), f + 0x214);
    let n = rd(0x30) as i32;
    if n - 1 > 0 {
        for i in 0..(n - 1) as usize {
            for j in i + 1..n as usize {
                let a = rd(0x14c + 4 * i);
                if a != 0 && a == rd(0x14c + 4 * j) {
                    show_error_at(0xa15da4, 0x4c9);
                }
            }
        }
        wd(0x20, n as u32);
    }

    // the rotation (0x8cf880 points out who rests; 0x6e8910 re-evaluates), at most two rounds
    if *(game.addr() as *const i32) != -2 {
        *((f + 0x2e) as *mut u8) = 0;
        let mut r = rodizio(u(G_TRANSFER_MANAGER, 0), club, game.addr(), 0, f + 0x14c, 2, f + 0x78);
        while r != 0 && *((f + 0x2e) as *mut u8) != 2 {
            let nb = *((f + 0x13) as *mut u8);
            *((f + 0x2e) as *mut u8) = (*((f + 0x2e) as *mut u8)).wrapping_add(1);
            core::ptr::copy((f + 0x14c) as *mut u8 , (f + 0x340) as *mut u8 , 44);
            let v = if nb as i8 > 0xb {
                let k = rd(0x30) - 0xb;
                let v = exe::crt_calloc(k, 4);
                core::ptr::copy((f + 0x178) as *mut u8 , v as *mut u8, 4 * k as usize);
                v
            } else {
                0
            };
            wd(0x24, v as u32);
            *((f + 0x20) as *mut u8) = 0;
            let w = if *((f + 0x18) as *mut u8) as i8 > nb as i8 {
                let w = exe::crt_calloc(0x32, 4);
                for c in 0..(*((f + 0x18) as *mut u8) as i8).max(0) as usize {
                    let p = rd(0x278 + 4 * c);
                    if p == 0 {
                        continue;
                    }
                    let inside = nb as i8 > 0 && (0..rd(0x30) as usize).any(|j| rd(0x14c + 4 * j) == p);
                    if !inside {
                        *((w + 4 * *((f + 0x20) as *mut u8) as i8 as usize) as *mut u32) = p;
                        *((f + 0x20) as *mut u8) = (*((f + 0x20) as *mut u8)).wrapping_add(1);
                    }
                }
                w
            } else {
                0
            };
            // simplification: the 6th argument carries only the byte (n - 11); the original passes ebx leftovers in the high bytes (read as a byte)
            exe::match_man_apply_squad_restriction(club, u(t.addr(), match_manager::MANAGER), f + 0x78, rd(0x14), 0xb, nb.wrapping_sub(0xb) as u32, rd(0x20), f + 0x340, v, w, f + 0x14c, f + 0xe8, f + 0x214, f + 0x5bc, f + 0xb4, 0);
            exe::crt_free(v);
            if w != 0 {
                exe::crt_free(w);
            }
            r = rodizio(u(G_TRANSFER_MANAGER, 0), club, game.addr(), 0, f + 0x14c, 2, f + 0x78);
        }
    }
    if mode as u8 == 1 {
        *((f + 0x12) as *mut u8) = 0;
        *((f + 0x24) as *mut u8) = 0;
        let nn = *((f + 0x13) as *mut u8) as i8;
        if nn > 0 {
            for k in 0..nn as usize {
                let p = if *((f + 0x90 + k) as *mut u8) != 0 {
                    exe::tactics_get_selected_player(tactic, club, rd(0x24), t.tactic_slot() as i32) as u32
                } else {
                    let c = *((f + 0x12) as *mut u8) as i8 as usize;
                    *((f + 0x12) as *mut u8) = (*((f + 0x12) as *mut u8)).wrapping_add(1);
                    rd(0x14c + 4 * c)
                };
                wd(0x36c + 4 * k, p);
                *((f + 0x24) as *mut u8) = (k + 1) as u8;
            }
            core::ptr::copy((f + 0x36c) as *mut u8 , (f + 0x14c) as *mut u8 , 4 * rd(0x30) as usize);
        }
    }

    // in the slots, the captain, the instructions
    let cap = exe::match_man_choose_captain(t, f + 0x14c, 0xb);
    let n = rd(0x30) as i32;
    for k in 0..n.max(0) as usize {
        exe::match_man_place_player_in_team(t, k as u32, rd(0x14c + 4 * k) as usize, k as u32);
        let p = rd(0x14c + 4 * k) as usize;
        if p != 0 {
            exe::tactics_set_selected_player(tactic, club, k as u32, p, humano19);
            *((f + 0x37) as *mut u8) = (*((f + 0x37) as *mut u8)).wrapping_add(1);
            if rd(0x14c + 4 * k) as usize == cap {
                capitao(tactic, club, k as u32, humano19);
            }
        }
    }
    let n_final = n.max(0) as usize;
    exe::match_man_set_computer_instructions(t);
    let name = if u(t.addr(), match_manager::TACTICS) != 0 { u(t.addr(), match_manager::TACTICS) } else { 0xa15934 };
    let name = core::ffi::CStr::from_ptr(name as *const core::ffi::c_char).to_bytes();
    let special = name != b"3-4-3" && name != b"3-5-2 Attacking";

    // the general orders by stance (0x5b50d0)
    let g = |x: u32| exe::formation_set_team_instruction(u(t.addr(), match_manager::TACTICS), x);
    let a = |x: u32| {
        g(x);
        g(0x800);
        g(0x2000);
    };
    let bx = |x: u32| {
        g(x);
        g(0x2000);
    };
    let cx = || {
        g(0x800);
        g(0x2000);
    };
    let sheet = u(t.addr(), match_manager::MANAGER_SHEET);
    let lim = |base: i32| if weight_of_side() > 0 { base + 3 } else { base };
    let disposition = |i: isize| *(((m as isize) + (engine::STANCE_FACTOR as isize) + 4 * i) as *const f32) as f64;
    let e = |o: usize| read_i8(sheet + o) as i32;
    match exe::match_man_choose_mentality(t) as i8 {
        1 => {
            let d = disposition(side);
            let x = if sheet != 0 && !(d >= read_f64(rdata::F64_2_5)) {
                if e(non_player::ATTACKING) > 6 && special { 0x10000 } else { 0x20000 }
            } else if special || d > read_f64(rdata::F64_10) {
                0x10000
            } else {
                0x20000
            };
            a(x);
        }
        3 => {
            if sheet == 0 {
                g(0x8000);
                bx(0x1000);
            } else {
                g(if e(non_player::ATTACKING) > 0x10 { 0x20000 } else { 0x8000 });
                if e(non_player::DIRECTNESS) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        4 => {
            if sheet != 0 {
                g(if e(non_player::ATTACKING) > 0x12 { 0x20000 } else { 0x8000 });
            } else {
                g(0x8000);
            }
            if sheet != 0 && e(non_player::ATTACKING) > 0xc {
                if e(non_player::DIRECTNESS) < lim(10) { bx(0x1000) } else { cx() }
            } else {
                g(0x800);
                g(0x4000);
            }
        }
        5 => {
            if sheet == 0 {
                g(0x20000);
                bx(0x1000);
            } else {
                g(if e(non_player::ATTACKING) > 0x10 { 0x10000 } else if e(non_player::ATTACKING) > 0xc { 0x20000 } else { 0x8000 });
                if e(non_player::DIRECTNESS) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        _ => {
            if sheet == 0 {
                a(0x20000);
            } else if e(non_player::ATTACKING) > lim(0xf) && special {
                a(0x10000);
            } else if e(non_player::ATTACKING) <= 5 && weight_of_side() != 0 && !(disposition(n_final as isize) > read_f64(rdata::F64_1)) {
                // as in the original: reads the formation at the index of the total slots (the loop’s ebx), not the side’s
                g(0x8000);
                if e(non_player::DIRECTNESS) < lim(4) { bx(0x1000) } else { cx() }
            } else {
                a(0x20000);
            }
        }
    }
    exe::drop_llist(f + 0x80);
    *((f + 0x37) as *mut u8) as i8
}

// simplification: allocates the list and the rotation arrays and calls 24 routines still original; no comparison
// own, validated by the career (Rust x original), like those of the 145th, 148th and 149th.

/// Match clock minute (`engine::CLOCK` / 11) capped at the end of the current period (`engine::PERIOD_END`: 45, 90, 105, 120).
pub(crate) unsafe fn minute_in_period(m: Engine) -> i8 {
    let min = (m.clock() as i32 / 11) as i8;
    match m.period_end() {
        0x1ef => min.min(0x2d),
        0x3de => min.min(0x5a),
        0x483 => min.min(0x69),
        0x528 => min.min(0x78),
        _ => min,
    }
}

/// { `exe::match_man_make_substitutions` } (ret 4, match_man.cpp) with the calls that touch the game injected (0x6e2000 lineup picker, 0x6d6f70
/// commentates, 0x897760 writes the captain, `redo_team` rebuilds the team): staff `c` makes up to +0x17 substitutions.
///
/// The eligible players (on the pitch, or −2 without +0x1e) go into a list sorted by the CRT's qsort (`exe::crt_qsort` with
/// 0x6e0df0, or 0x6e0fe0 with 11 or more); the first `m` give the slots (tables `exe::match_player_get_alt_slot_position` for the −2 ones,
/// `player_table`/`exe::match_player_get_secondary_position`; position +0x19 or +0x1a; the first is 0 with `goleiro`; marks `exe::formation_get_player_instructions` and byte `position_byte` of the
/// position); after them, the free substitutes (+0x19 = −1) from the 12th on, up to 11 + (competition +0x4d & 0xf),
/// enter as candidates.
///
/// The lineup picker's mode comes from the club (`exe::database_is_national_team`), the strength against the opponent (+0x80, with random
/// draws), the competition (+0x3a, +0x1a = 3 and `exe::comp_is_club_still_in_contention`), +0x16 and the human manager. Whoever wins a slot and was on
/// the bench comes on (`exe::cm3_match_engine_place_player_at_entry_cell`, −1 at +0x17, minute at +0x1f) with the slot's position; whoever is left without a slot
/// goes off (`exe::cm3_match_engine_remove_player_from_pitch` with −1, +0x19 = −4, +0x1e = 1, minute at +0x20).
///
/// It recomposes the positions (`exe::cm3_match_engine_clear_goalkeeper`, `exe::cm3_match_engine_get_goalkeeper`) and commentates the substitutions (0x1f59 for more than 2, 0x1f5c
/// for two, 0x1f5a for one; with no substitution it stops), each with the fatigue comments (0x211d by random
/// draw(+0x31) below −10) and injury comments (0x211e) of the outgoing and incoming players, and 0x20d7 or 0x1f5b
/// (negative +0x22 of the outgoing one). Finally the best-rated player (`exe::match_man_get_captain_score`) becomes captain and the team is
/// rebuilt.
pub(crate) unsafe fn substitute_with(c: MatchManager, goalkeeper: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let swaps = c.substitutions_left();
    if swaps == 0 {
        return;
    }
    let mut list = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = c.player(k).addr();
        let e = read_i8(j + match_player::POSITION);
        if e >= 0 || (e == -2 && read_i8(j + match_player::SUBSTITUTED_OFF) == 0) {
            list[n] = j;
            n += 1;
        }
    }
    qsort(list.as_mut_ptr() as usize, n as u32, 4, if n < 11 { 0x6e0df0 } else { 0x6e0fe0 });
    let slots = (swaps as i32).min(n as i32).max(0) as usize;
    let mut table = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut pos = [0i8; 20];
    let mut mark = [0u64; 20];
    let mut bpos = [0u8; 20];
    for i in 0..slots {
        let p = list[i];
        if read_i8(p + match_player::POSITION) == -2 {
            table[i] = exe::match_player_get_alt_slot_position(p);
            tab_b[i] = 0;
        } else {
            table[i] = exe::match_player_get_position(p);
            tab_b[i] = exe::match_player_get_secondary_position(p);
        }
        pos[i] = if i == 0 && goalkeeper != 0 {
            0
        } else if read_i8(p + match_player::POSITION) == -2 {
            read_i8(p + match_player::PREVIOUS_POSITION)
        } else {
            read_i8(p + match_player::POSITION)
        };
        mark[i] = exe::formation_get_player_instructions(c.tactics(), pos[i]);
        bpos[i] = exe::formation_get_marking_type(c.tactics(), pos[i]);
    }
    let comp = c.fixture();
    // the substitutes come right after the slots (the rest of the sorted list is left out)
    let mut n1 = slots;
    let limit = ((comp.squad_rules() as u8) & 0xf) as i32 + 0xb;
    if limit > 0xb {
        let mut p = c.players() + 0x132a;
        let mut k = 0xb;
        while k < limit {
            if p != 0 && read_i8(p + match_player::POSITION) == -1 {
                list[n1] = p;
                n1 += 1;
            }
            k += 1;
            p += 0x1be;
        }
    }
    let mut sheets = [0usize; 20];
    for i in 0..n1 {
        sheets[i] = read_usize(list[i] + match_player::STAFF);
    }
    // the scale mode
    let club = c.club();
    let strength = |o: usize| read_i16(read_usize(c.addr() + o) + club::REPUTATION) as i32;
    let mut mode: u32 = if exe::database_is_national_team(club) != 0 { 0x1ffffd2 + 0x8a0cb } else { 0x8a0cb };
    if c.tactic_slot() != 0 {
        mode = (mode | 0x20100) ^ 0x80000;
    } else {
        let mut done = false;
        if matches!(comp.category() as i8 , 1 | 9) {
            let r = random(0x7d0).wrapping_add(random(-4000));
            if strength(match_manager::CLUB) - strength(match_manager::OPPONENT) > r {
                mode |= 0x100;
                done = true;
            }
        }
        if !done {
            let r = random(0xbb8) + 0xbb8;
            let check_national_team = if strength(match_manager::CLUB) - strength(match_manager::OPPONENT) > r {
                mode |= 0x100;
                true
            } else if c.selection_mode() != rotation_mode::NO_ROTATION {
                false
            } else {
                let id = read_i32(comp.addr());
                id <= -1 || exe::comp_is_club_still_in_contention(read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4), club) != 0
            };
            if check_national_team {
                if exe::database_is_national_team(club) == 0 {
                    mode |= 0x10000;
                }
            } else {
                mode = (mode | 0x100) ^ 0x80000;
            }
        }
    }
    if c.selection_flag() != 0 {
        mode |= 0x800000;
    }
    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 && c.tactic_slot() == 0 {
        let p = club.manager().addr();
        if p != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), p) != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), (3) as u32, p) != 0 {
            mode |= 0x400000;
        }
    }
    let mut place = [0usize; 20];
    let extra = read_u32(c.tactics() + formation_fields::FLAGS);
    let args: [usize; 15] = [
        sheets.as_ptr() as usize, n1, table.as_ptr() as usize, slots, 0, place.as_mut_ptr() as usize, mode as usize, club.addr(), c.addr(), c.manager(), 0,
        mark.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    request_buffers(
        &[(args[0], 4 * n1), (args[2], 2 * slots), (args[11], 8 * slots), (args[12], slots), (args[14], 2 * slots)],
        &[(args[5], 4 * slots)],
    );
    call(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS, 0, [args.as_ptr() as u32, n1 as u32, mode, club.addr() as u32, c.addr() as u32]);
    let m = c.engine();
    let mut entraram = [0usize; 20];
    let mut sairam = [0usize; 20];
    let (mut ne, mut ns) = (0usize, 0usize);
    for i in 0..n1 {
        let mut j = c.players();
        let mut found = false;
        for _ in 0..20 {
            if read_usize(j + match_player::STAFF) == sheets[i] {
                found = true;
                break;
            }
            j += 0x1be;
        }
        if !found || j == 0 {
            write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
            continue;
        }
        let slot = (0..slots).find(|&k| place[k] == read_usize(j + match_player::STAFF));
        match slot {
            Some(k) => {
                if read_i8(j + match_player::POSITION) == -1 {
                    c.set_substitutions_left(c.substitutions_left().wrapping_sub(1));
                    exe::cm3_match_engine_place_player_at_entry_cell(m, j);
                    write_i8(j + match_player::MINUTE_ON, minute_in_period(m));
                    entraram[ne] = j;
                    ne += 1;
                }
                let e = read_i8(j + match_player::POSITION);
                if e >= 0 || e == -1 {
                    write_i8(j + match_player::POSITION, pos[k]);
                }
            }
            None => {
                if read_i8(j + match_player::POSITION) == -1 {
                    continue;
                }
                exe::cm3_match_engine_remove_player_from_pitch(m, j, -1);
                write_i8(j + match_player::POSITION, -4);
                write_u8(j + match_player::SUBSTITUTED_OFF, 1);
                write_i8(j + match_player::MINUTE_OFF, minute_in_period(m));
                sairam[ns] = j;
                ns += 1;
            }
        }
    }
    exe::cm3_match_engine_clear_goalkeeper(m, c.side());
    exe::cm3_match_engine_get_goalkeeper(m, c.side());
    let commentator = m.commentator();
    let side_c = c.side() as u8 as u32;
    let code = match ne {
        0 => return,
        1 => 0x1f5a,
        2 => 0x1f5c,
        _ => 0x1f59,
    };
    call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [code, m.addr() as u32, u32::MAX, side_c, u32::MAX]);
    for i in 0..ne {
        let (sai, entra) = (sairam[i], entraram[i]);
        let comentarios = m.live_match() != 0;
        // fatigue: first the one going off, then the one coming on
        let tired = if comentarios && random(read_i16(sai + match_player::MATCH_TEMPER) as i32) < -10 {
            Some(sai)
        } else if comentarios && random(read_i16(entra + match_player::MATCH_TEMPER) as i32) < -10 {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = tired {
            call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [event::BOO_SPECIFIC_PLAYER, m.addr() as u32, u32::MAX, (read_i8(p + match_player::SIDE) == 0) as u32, u32::MAX]);
        }
        let injured = if comentarios && {
            let r = random(read_i16(sai + match_player::MATCH_RATING) as i32 / 30 + read_i16(sai + match_player::OWN_CLUB_FEELING) as i32 - 0xe9);
            r > random(0x1e)
        } {
            Some(sai)
        } else if comentarios && {
            let r = random(read_i16(entra + match_player::OWN_CLUB_FEELING) as i32);
            r > random(0x1e)
        } {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = injured {
            call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [event::APPLAUD_SPECIFIC_PLAYER, m.addr() as u32, u32::MAX, read_i8(p + match_player::SIDE) as u8 as u32, u32::MAX]);
        }
        let cod = if read_i8(sai + match_player::INJURY_MINUTE) > -1 { 0x20d7 } else { 0x1f5b };
        let a = [cod, m.addr() as u32, read_i8(entra + match_player::COMMENTARY_ID) as u8 as u32, read_i8(entra + match_player::SIDE) as u8 as u32, read_i8(sai + match_player::COMMENTARY_ID) as u8 as u32];
        NEXT_SIXTH_ARG.store(read_i8(sai + match_player::SIDE) as u8 as u32, std::sync::atomic::Ordering::Relaxed);
        call(FN_MATCH_EVENTS_ADD_EVENT, commentator, a);
    }
    // o capitao
    let (mut melhor, mut nota) = (-1i8, 0i16);
    let mut j = c.players();
    for k in 0..20i8 {
        if read_i8(j + match_player::POSITION) >= 0 {
            let v = exe::match_man_get_captain_score(0, j, club);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        j += 0x1be;
    }
    call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, read_usize(G_TACTICS), [club.addr() as u32, melhor as u8 as u32, c.tactic_slot() as i32 as u32, 0, 0]);
    call(FN_MATCH_MAN_REASSIGN_POSITIONS, c.addr(), [0, 0, 0, 0, 0]);
}

/// O 6o argumento da proxima narracao (o lado de quem sai em 0x20d7/0x1f5b; -1 nas outras).
pub(crate) static NEXT_SIXTH_ARG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

/// Makes the AI substitutions for a side (choosing who comes on and off, reassigning positions, commentary, new
/// captain); thin wrapper over substitute_with with the real calls.
pub(crate) unsafe extern "thiscall" fn substitute(c: usize, goalkeeper: u32) {
    substitute_with(MatchManager(c), goalkeeper as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e2000 => {
                let f: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
                let v = *(a[0] as usize as *const [usize; 15]);
                return f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14]);
            }
            0x897760 => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            _ => {
                let sexto = NEXT_SIXTH_ARG.swap(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                add_match_event(this, [a[0], a[1], a[2], a[3], a[4], sexto, 0]);
            }
        }
        0
    })
}

/// Argumentos da 0x6e2000 (escalar: distribui as pessoas pelas posicoes, recursiva, aloca), cdecl.
pub(crate) type AssignPositionsFn = unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u32;

/// `0x6e1aa0` (match_man.cpp) with the calls that change the game injected (`exe::match_man_choose_computer_tactic` judges the line-up, 0x6e2000 picks
/// the team, 0x6d6f70 commentary): the staff (`c`: +0x12 engine, +0x23 first of the 20 players, +0x2b tactic, +0x2f
/// club) rebuild the team (formation check, selection, new positions and ratings for each player).
///
/// With exactly 11 on the pitch and the line-up rejected (`exe::match_man_choose_computer_tactic` with 0x8c1, 2) it comments 0x2007.
///
/// Then it gathers the outfield players on the pitch (except table 1): position +0x19, tables `player_table`/`exe::match_player_get_secondary_position`, marks
/// (`exe::formation_get_player_instructions`) and byte (`position_byte`) of the position and the player sheet +0x69; picks the team (0x6e2000 with 0x8800, or
/// 0x408800 with a human manager to be consulted: `exe::human_manager_is_human_club`, the staff's +0x19, the club's +0xcf, `exe::human_manager_is_on_holiday`, `exe::human_manager_get_holiday_option`
/// with 3) and gives each selected player sheet the position of the place it won, recomputing its performance
/// (`exe::match_player_calc_ratings`).
pub(crate) unsafe fn rebuild_team_with(c: MatchManager, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mut sheets = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = c.player(k).addr();
        if read_i8(j + match_player::POSITION) >= 0 {
            sheets[n] = read_usize(j + match_player::STAFF);
            n += 1;
        }
    }
    if n == 11 {
        request_buffers(&[(sheets.as_ptr() as usize, 44)], &[]);
        if call(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC, c.addr(), [sheets.as_ptr() as u32, 0xb, 0x8c1, 2, 0]) as u8 != 0 {
            let m = c.engine();
            call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::CHANGE_FORMATION, m.addr() as u32, u32::MAX, c.side() as u8 as u32, u32::MAX]);
        }
    }
    let mut pos = [0i8; 20];
    let mut table = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut mark = [0u64; 20];
    let mut bpos = [0u8; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = c.player(k).addr();
        if read_i8(j + match_player::POSITION) < 0 || exe::match_player_get_position(j) == 1 {
            continue;
        }
        pos[n] = read_i8(j + match_player::POSITION);
        table[n] = exe::match_player_get_position(j);
        tab_b[n] = exe::match_player_get_secondary_position(j);
        mark[n] = exe::formation_get_player_instructions(c.tactics(), read_i8(j + match_player::POSITION));
        bpos[n] = exe::formation_get_marking_type(c.tactics(), read_i8(j + match_player::POSITION));
        sheets[n] = read_usize(j + match_player::STAFF);
        n += 1;
    }
    let club = c.club();
    let mut mode = 0x8800u32;
    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 && c.tactic_slot() == 0 {
        let p = club.manager().addr();
        if p != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), p) != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), (3) as u32, p) != 0 {
            mode = 0x408800;
        }
    }
    let mut place = [0usize; 20];
    let extra = read_u32(c.tactics() + formation_fields::FLAGS);
    let args: [usize; 15] = [
        sheets.as_ptr() as usize, n, table.as_ptr() as usize, n, 0, place.as_mut_ptr() as usize, mode as usize, club.addr(), c.addr(), c.manager(), 0,
        mark.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    request_buffers(
        &[(args[0], 4 * n), (args[2], 2 * n), (args[11], 8 * n), (args[12], n), (args[14], 2 * n)],
        &[(args[5], 4 * n)],
    );
    call(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS, 0, [args.as_ptr() as u32, n as u32, mode, club.addr() as u32, c.addr() as u32]);
    for i in 0..n {
        let mut j = c.players();
        let mut found = false;
        for _ in 0..20 {
            if read_usize(j + match_player::STAFF) == sheets[i] {
                found = true;
                break;
            }
            j += 0x1be;
        }
        if !found {
            j = 0;
        }
        let f = read_usize(j + match_player::STAFF);
        if let Some(k) = (0..n).find(|&k| place[k] == f) {
            write_i8(j + match_player::POSITION, pos[k]);
            exe::match_player_calc_ratings(j, 0);
        }
    }
}

/// the staff redo the team. With 11 on the pitch and the line-up rejected (`exe::match_man_choose_computer_tactic` with 0x8c1, 2) narrates 0x2007.
/// Gathers the outfield players on the pitch (position +0x19, tables `player_table`/`exe::match_player_get_secondary_position`, position marks `exe::formation_get_player_instructions` and
/// byte `position_byte`, record +0x69), picks the team (0x6e2000 with 0x8800, or 0x408800 when a human manager is to be
/// consulted: `exe::human_manager_is_human_club`, `exe::human_manager_is_on_holiday`, `exe::human_manager_get_holiday_option`) and gives each picked record the position of the place it won, redoing
/// performance (`exe::match_player_calc_ratings`). `exe::match_man_choose_computer_tactic` and 0x6e2000 (pick, recursive, allocates) are units with vectors: the tape records
/// the bytes of the input and output vectors passed by pointer, checks the Rust copy's inputs and writes its outputs
/// (`call_recording_buffers`, `request_buffers`). The provisional `redo_team` port from batch 114 is gone
/// Original: `redo_team` (0x6e1aa0, match_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn redo_team(c: usize) {
    rebuild_team_with(MatchManager(c), &mut |rotina, this, a| match rotina {
        0x6e0230 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC);
            f(this, a[0] as usize, a[1], a[2], a[3]) as u32
        }
        0x6e2000 => {
            let f: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
            let v = *(a[0] as usize as *const [usize; 15]);
            f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14])
        }
        _ => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

/// Among the `n` people in `lista` with a player record (+0x61), returns the one best rated by the staff of club +0x2f
/// (captain score `exe::match_man_get_captain_score`); with the object at +0x1f, a rating may drop to (1.1 - 0.01 x (object address mod 20)) x
/// rating when its attribute is below the person's id mod 20, as in the original. Ties keep the first; 0 if none.
pub(crate) unsafe extern "thiscall" fn best_in_list(this: usize, list: *const usize, n: i8) -> usize {
    let this = MatchManager(this);
    let (mut melhor, mut nota) = (0usize, 0i16);
    for i in 0..n.max(0) as usize {
        let p = Staff(*list.add(i));
        if p.is_null() || p.player().is_null() {
            continue;
        }
        let mut v = exe::match_man_get_captain_score(p, 0, this.club()) as i32;
        let x = this.manager_sheet();
        if !x.is_null() && ((exe::non_player_attribute_rating(x, (x.judgement() as u8 as u32) as i8) as i8) as i32) < read_i32(p.addr()) % 20 {
            let m = (x.addr() as i32) % 20;
            v = ftol((read_f64(rdata::F64_1_1) - m as f64 * read_f64(rdata::F64_0_01)) * v as i16 as f64);
        }
        if melhor == 0 || v as i16 > nota {
            melhor = p.addr();
            nota = v as i16;
        }
    }
    melhor
}

/// the index (0..20) of the active player (+0x19 ≥ 0) best rated by the staff (`exe::match_man_get_captain_score` with club +0x2f) among the 20
/// in the block starting at +0x23; the first active one counts even with a negative rating, then only strictly higher;
/// −1 with none active
/// Original: `best_rated` (0x6e1a40, match_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn best_rated(this: usize) -> i8 {
    let this = MatchManager(this);
    let club = this.club();
    let mut p = this.players();
    let (mut melhor, mut nota) = (-1i8, 0i16);
    for k in 0..20i8 {
        if read_i8(p + match_player::POSITION) >= 0 {
            let v = exe::match_man_get_captain_score(0, p, club);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        p += 0x1be;
    }
    melhor
}

/// `exe::match_man_get_captain_score` (cdecl, match_man.cpp): the rating (i16) the staff give `pessoa` (or the person of match player `jogador`
/// +0x69) for `clube`; -10000 with no person.
///
/// It adds the fit (`exe::database_get_contract_rating_modifier`) as (fit^2 + 5) x 0.1 negative, the difference between the player sheet's rating +0xd and
/// the club's reputation (x 0.002 + 1.5, between -4 and 4), the contract (`exe::contract_manager_get_contract_for_club`: +0x44 / 75 + 0.5, adjusted by the
/// type in +0x4f >> 4: -10, +2.5, +2, -3, -15, -20, -50) and the player sheet's +0x2f times the age factor (days of
/// life from today's date 0xae2c90 and the birth date +0x3e, `exe::date_days_since`; for national teams, +0x22 x 75): above 1000
/// days, 1 + (days - 1000) x 0.000125.
///
/// Then by age +0x18 (x0.5 below 18, x0.75 below 21, x0.9 below 24, x1.05 above 25), x0.9 if the days fall below (50 -
/// +0x59) x 10, and x(0.01 x +0x5b + 0.9) with +0x5b below 10.
pub(crate) unsafe extern "C" fn rate_player(person: usize, player: usize, club: usize) -> i16 {
    let player = MatchPlayer(player);
    let club = Club(club);
    let encaixe: unsafe extern "C" fn(usize, usize) -> i8 = core::mem::transmute(FN_DATABASE_GET_CONTRACT_RATING_MODIFIER);
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let p = Staff(if !player.is_null() { player.staff().addr() } else { person });
    if p.is_null() {
        return -10000;
    }
    let sheet = p.player();
    let enc = encaixe(p.addr(), club.addr()) as f64;
    let mut y = (sheet.world_reputation() as i32 - club.reputation() as i32) as f64 * read_f64(rdata::F64_0_002) + read_f64(rdata::F64_1_5);
    if !(read_f64(rdata::F64_NEG_4) <= y) {
        y = -4.0;
    } else if read_f64(rdata::F64_4) < y {
        y = 4.0;
    }
    let mut x = 0.0f64;
    let e = Contract(contract(read_usize(G_CONTRACT_MANAGER), p.addr(), club.addr()));
    if !e.is_null() {
        x = e.morale() as f64 * read_f64(rdata::F64_0_01333333333) + read_f64(rdata::F64_0_5);
        let adjust = match (e.squad_status() as i8) as u8 >> 4 {
            0 | 5 => -read_f64(rdata::F64_10),
            1 => read_f64(rdata::F64_2_5),
            2 => read_f64(rdata::F64_2),
            4 => -read_f64(rdata::F64_3),
            6 => -read_f64(rdata::F64_15),
            7 => -read_f64(rdata::F64_20),
            8 => -read_f64(rdata::F64_50),
            _ => 0.0,
        };
        if adjust != 0.0 {
            x += adjust;
        }
    }
    let a = if exe::database_is_national_team(club) != 0 {
        p.int_apps() as f64 * read_f64(rdata::F64_75)
    } else {
        exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32 , p.date_joined_club_leap_year() as u32) as f64
    };
    let f = if a > read_f64(rdata::F64_1000) { (a - read_f64(rdata::F64_1000)) * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1) } else { 1.0 };
    let base = sheet.leadership() as f64 * f - (enc * enc + read_f64(rdata::F64_5)) * read_f64(rdata::F64_0_1);
    let mut v = ftol(base + y + x + read_f64(rdata::F64_0_5)) as i16;
    let age = p.year_of_birth() as i8;
    let k = if age < 0x12 {
        Some(read_f64(rdata::F64_0_5))
    } else if age < 0x15 {
        Some(read_f64(rdata::F64_0_75))
    } else if age < 0x18 {
        Some(read_f64(rdata::F64_0_9))
    } else if age > 0x19 {
        Some(read_f64(rdata::F64_1_05))
    } else {
        None
    };
    if let Some(k) = k {
        v = ftol(v as f64 * k) as i16;
    }
    if a < ((0x32 - p.loyalty() as i32) * 10) as f64 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_9)) as i16;
    }
    let morale = p.professionalism();
    if morale < 10 {
        v = ftol((morale as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_9)) * v as f64) as i16;
    }
    v
}

/// Applies the squad rule (foreigners, non-EU or age) to the club's lineup: swaps restricted players for the candidates
/// with the smallest score loss.
///
/// Enforces squad rule `regra` (byte 0: 1 foreigner from outside the community, 2 foreigner, 3 over the age in `+1`;
/// `+2` maximum on the pitch, `+3` maximum on the bench, `+4` and `+5` the allowed and the current) on the lineup of
/// `clube` (nation `+0x53`; `0x53d980`/`0x53d740`): removes the restricted ones from the reserve list (`reserva`,
/// `nr`); if there are more restricted players on the pitch than the maximum (`+2`), it chooses among the restricted
/// starters and the candidates (non-restricted bench + reserve) via `0x6e2000` and swaps, one by one, the one with the
/// smallest score loss (`0x6e2f20`), redoing the bench (`0x6e2000`) if a starter came from it; then it does the same on
/// the bench (`saida` + `nt`) with the maximum `+3` (or `+4` − `+5`).
/// Original: `apply_squad_rule` (0x6e8910, match_man.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn apply_squad_rule(
    club: usize, manager: usize, rule: usize, mode: u32, nt: u32, nb: u32, nr: u32, starters: usize, bench: usize, reserve: usize,
    out: usize, codes: usize, curtos: usize, marks: usize, bytes: usize, side: usize,
) {
    let side = MatchManager(side);
    let club = Club(club);
    if read_i8(rule + 0) == 0 || read_i8(rule + 5) >= read_i8(rule + 4) {
        return;
    }
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let restrito = |p: Staff| -> bool {
        let nation = club.nation().addr() as u32;
        match read_i8(rule + 0) {
            1 => !p.is_null() && exe::database_is_staff_foreign(p, (nation) as usize, 1) != 0 && exe::database_is_staff_eu_national(p, (nation) as usize) == 0,
            2 => !p.is_null() && exe::database_is_staff_foreign(p, (nation) as usize, 1) != 0,
            3 => !p.is_null() && read_i8(rule + 1) < (p.year_of_birth() as i8),
            _ => false,
        }
    };
    let (nt, nb) = (nt as u8 as i8, nb as u8 as i8);
    let mut nr = nr as u8 as i8;
    let res = |k: i8| (reserve as isize + 4 * k as isize) as *mut usize;
    let sai = |k: i32| (out as isize + 4 * k as isize) as *mut usize;
    let w = |base: usize, k: i32| *((base as isize + 2 * k as isize) as *const u16);
    // removes `alvo` (or position `k`) from the reserves, putting the last one in its place
    let remove = |k: i8, nr: &mut i8| {
        if (k as i32) < *nr as i32 - 1 {
            *res(k) = *res(*nr - 1);
            *res(*nr - 1) = 0;
        } else {
            *res(k) = 0;
        }
        *nr -= 1;
    };
    let remove_target = |target: usize, nr: &mut i8| {
        let mut k = 0i8;
        if 0 < *nr {
            loop {
                if *res(k) == target {
                    remove(k, nr);
                    break;
                }
                k += 1;
                if k >= *nr {
                    break;
                }
            }
        }
    };
    let mark = if !side.is_null() { *((side.tactics() + formation_fields::FLAGS) as *const u32) } else { 0 };
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserve == 0 {
                break;
            }
            if restrito(Staff(*res(k))) {
                let last = (k as i32) < nr as i32 - 1;
                remove(k, &mut nr);
                if last {
                    k -= 1;
                }
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
    }
    let mut slot = [0i8; 52];
    let mut code = [0u16; 50];
    let mut cod2 = [0u16; 50];
    let mut m8 = [0u32; 100];
    let mut b1 = [0u8; 52];
    let mut cand = [0usize; 50];
    let mut choice = [0usize; 50];
    let mut foul = read_i8(rule + 5);
    if foul < read_i8(rule + 2) {
        let mut swaps = read_i8(rule + 2) - foul;
        foul = read_i8(rule + 3);
        if swaps != 0 {
            let mut nv = 0i8;
            let mut k = 0i8;
            while k < nt {
                if restrito(Staff(*((starters + 4 * k as usize) as *const usize))) {
                    let i = nv as usize;
                    slot[i] = k;
                    code[i] = w(codes, k as i32);
                    cod2[i] = if curtos == 0 { 0 } else { w(curtos, k as i32) };
                    (m8[2 * i], m8[2 * i + 1]) = if marks == 0 { (0, 0) } else { (*((marks + 8 * k as usize) as *const u32), *((marks + 8 * k as usize + 4) as *const u32)) };
                    b1[i] = if bytes == 0 { 0 } else { *((bytes + k as usize) as *const u8) };
                    nv += 1;
                }
                k += 1;
            }
            let mut nc = 0i8;
            for k in 0..nb.max(0) as usize {
                let q = *((bench + 4 * k) as *const usize);
                if q != 0 && !restrito(Staff(q)) {
                    cand[nc as usize] = q;
                    nc += 1;
                }
            }
            let mut k = 0i8;
            if 0 < nr {
                loop {
                    if reserve == 0 {
                        break;
                    }
                    cand[nc as usize] = *res(k);
                    nc += 1;
                    k += 1;
                    if k >= nr {
                        break;
                    }
                }
            }
            if nc != 0 {
                pick(cand.as_ptr() as usize, nc as u8 as u32, code.as_ptr() as usize, nv as u8 as u32, 0, choice.as_mut_ptr() as usize, mode, club.addr(), side.addr(),
                    manager as u32, 0, m8.as_ptr() as usize, b1.as_ptr() as usize, mark, cod2.as_ptr() as usize);
                let mut best = 0i32;
                let mut left_of_bench = false;
                loop {
                    if nv == 0 || nv < 1 {
                        break;
                    }
                    let mut esc = -1i32;
                    for k in 0..nv as usize {
                        if slot[k] == -1 {
                            continue;
                        }
                        let current = *sai(slot[k] as i32);
                        if current == 0 {
                            continue;
                        }
                        let valer = |p: Staff| rating(p.addr(), p.player().addr(), 0, code[k] as u32, mode, club.addr(), side.addr(), manager as u32, u32::MAX,
                            m8[2 * k], m8[2 * k + 1], b1[k] as u32, mark, cod2[k] as u32, 0);
                        let a = valer(Staff(current));
                        let b = if choice[k] == 0 { -1000000 } else { valer(Staff(choice[k])) };
                        if esc == -1 || a.wrapping_sub(b) < best {
                            best = a.wrapping_sub(b);
                            esc = k as i32;
                        }
                    }
                    if esc < 0 {
                        break;
                    }
                    let e = esc as usize;
                    *sai(slot[e] as i32) = choice[e];
                    remove_target(choice[e], &mut nr);
                    for k in 0..nb.max(0) as usize {
                        let q = (bench + 4 * k) as *mut usize;
                        if choice[e] != 0 && *q == choice[e] {
                            left_of_bench = true;
                            *q = 0;
                        }
                    }
                    slot[e] = -1;
                    swaps -= 1;
                    if swaps == 0 {
                        break;
                    }
                }
                if left_of_bench {
                    let mut n = 0i8;
                    for k in 0..nb.max(0) as usize {
                        let q = *((bench + 4 * k) as *const usize);
                        if q != 0 {
                            choice[n as usize] = q;
                            n += 1;
                        }
                    }
                    for k in 0..nr.max(0) {
                        if *res(k) != 0 {
                            choice[n as usize] = *res(k);
                            n += 1;
                        }
                    }
                    if n != 0 {
                        pick(choice.as_ptr() as usize, n as u8 as u32, codes + 2 * nt as isize as usize, nb as u8 as u32, 0, sai(nt as i32) as usize, mode,
                            club.addr(), side.addr(), manager as u32, 0, 0, 0, 0, curtos.wrapping_add(2 * nt as isize as usize));
                    }
                }
            }
        }
    } else {
        foul = read_i8(rule + 4) - foul;
    }
    if foul == 0 {
        return;
    }
    let mut nv = 0i8;
    let mut k = 0i8;
    while k < nb {
        let j = nt as i32 + k as i32;
        if restrito(Staff(*sai(j))) {
            let i = nv as usize;
            code[i] = w(codes, j);
            cod2[i] = w(curtos, j);
            slot[i] = k;
            nv += 1;
        }
        k += 1;
    }
    let mut nc = 0i8;
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserve == 0 {
                break;
            }
            let q = *res(k);
            if !(0..nb.max(0) as i32).any(|j| q == *sai(j + nt as i32)) {
                cand[nc as usize] = q;
                nc += 1;
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
        if nc != 0 {
            pick(cand.as_ptr() as usize, nc as u8 as u32, code.as_ptr() as usize, nv as u8 as u32, 0, choice.as_mut_ptr() as usize, mode, club.addr(), side.addr(),
                manager as u32, 0, 0, 0, 0, cod2.as_ptr() as usize);
            let mut best = 0i32;
            loop {
                if nv == 0 || nv < 1 {
                    return;
                }
                let mut esc = -1i32;
                for k in 0..nv as usize {
                    if slot[k] == -1 {
                        continue;
                    }
                    let current = *sai(slot[k] as i32 + nt as i32);
                    if current == 0 {
                        continue;
                    }
                    let valer = |p: Staff| rating(p.addr(), p.player().addr(), 0, code[k] as u32, mode, club.addr(), side.addr(), manager as u32, u32::MAX, 0, 0, 0, 0, cod2[k] as u32, 0);
                    let a = valer(Staff(current));
                    let b = if choice[k] == 0 { -1000000 } else { valer(Staff(choice[k])) };
                    if esc == -1 || a.wrapping_sub(b) < best {
                        best = a.wrapping_sub(b);
                        esc = k as i32;
                    }
                }
                if esc < 0 {
                    return;
                }
                let e = esc as usize;
                *sai(slot[e] as i32 + nt as i32) = choice[e];
                remove_target(choice[e], &mut nr);
                slot[e] = -1;
                foul -= 1;
                if foul == 0 {
                    break;
                }
            }
        }
    }
}

// ---- the player ratings from match_man.cpp (0x6e6340 outside the match, 0x6e2f20 for a slot): common parts ----

/// Date `a` (day +0 signed, year +2 unsigned) is before date `b`.
pub(crate) unsafe fn match_man_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_u16(a + 2), read_u16(b + 2));
    aa < ab || (aa == ab && *(a as *const i16) < read_i16(b))
}

/// Days from the person's date (+0x3e/+0x40) to today, counting 365 per year.
pub(crate) unsafe fn match_man_days_since_date(p: Staff) -> i32 {
    let years = read_u16(G_CURRENT_DATE_YEAR) as i32 - p.date_joined_club_year() as i32;
    years * 365 - (p.date_joined_club() as i16) as i32 + read_i16(G_CURRENT_DATE) as i32
}

/// Observer's date (+0x3e) is before the player's, and the player's date is more than a year ago.
pub(crate) unsafe fn scouted_over_a_year_ago(o: Staff, p: Staff) -> bool {
    if !match_man_date_is_before(o.addr() + staff::DATE_JOINED_CLUB, p.addr() + staff::DATE_JOINED_CLUB) {
        return false;
    }
    let date: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> *const [u8; 8] = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    let r = date(G_CURRENT_DATE, &mut d, 0x16d);
    match_man_date_is_before(r as usize, p.addr() + staff::DATE_JOINED_CLUB)
}

/// Signed byte at p + k as i32.
pub(crate) unsafe fn signed_byte_at(p: usize, k: usize) -> i32 {
    read_i8(p + k) as i32
}

/// PLAYERSTATS get_stat (`get_stat` on G_PLAYER_STATS) for player data `f`.
pub(crate) unsafe fn player_stat(f: PlayerSheet, k: u32, c: u32, x: f32) -> f64 {
    exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), f, k, c, x)
}

/// Rating of player data `f` for position mask `m` from its attributes via table `tab`, with the main-position special
/// cases.
pub(crate) unsafe fn position_rating(f: PlayerSheet, m: u32, masc: u32, table: usize, main: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let t = |k: i32| *((table as isize + 4 * k as isize) as *const i32);
    let m16 = m as u16;
    let without_ala = m & 0x800 == 0 && m & 0x80 == 0;
    if m & 1 != 0 {
        fb(player_sheet::GOALKEEPER) * fb(player_sheet::GOALKEEPER) - 300
    } else if masc & 1 != 0 {
        -200
    } else if m & 2 != 0 {
        t(fb(player_sheet::SWEEPER))
    } else if m & 4 != 0 {
        t(fb(player_sheet::DEFENDER))
    } else if main && m16 == 8 {
        t(fb(player_sheet::WING_BACK).max(fb(player_sheet::DEFENDER)).max(fb(player_sheet::DEFENSIVE_MIDFIELDER)).max(fb(player_sheet::MIDFIELDER)))
    } else if m & 8 != 0 {
        t(if without_ala { fb(player_sheet::DEFENSIVE_MIDFIELDER) } else { fb(player_sheet::WING_BACK) })
    } else if m & 0x10 != 0 {
        if !main || without_ala {
            if main && m16 == 0x10 {
                t(fb(player_sheet::ATTACKING_MIDFIELDER).max(fb(player_sheet::DEFENSIVE_MIDFIELDER)).max(fb(player_sheet::MIDFIELDER)))
            } else {
                let (a, c, d) = (fb(player_sheet::ATTACKING_MIDFIELDER) - 5, fb(player_sheet::MIDFIELDER), fb(player_sheet::DEFENSIVE_MIDFIELDER) - 5);
                let al = if a.max(c) > d { if a > c { a } else { c } } else { d };
                t(al as i8 as i32)
            }
        } else {
            t(fb(player_sheet::ATTACKING_MIDFIELDER).max(fb(player_sheet::MIDFIELDER)))
        }
    } else if m & 0x20 != 0 {
        if without_ala {
            let a = fb(player_sheet::ATTACKING_MIDFIELDER) as f64;
            let middle = (fb(player_sheet::ATTACKING_MIDFIELDER) + fb(player_sheet::FREE_ROLE)) as f64 * read_f64(rdata::F64_0_5);
            t(ftol(if a <= middle { middle } else { a }))
        } else {
            t(fb(player_sheet::ATTACKING_MIDFIELDER))
        }
    } else if m & 0x40 != 0 {
        t(fb(player_sheet::ATTACKER))
    } else {
        100
    }
}

/// Adjustment of the position mask by the player's game attributes; 100 when no bit matches.
pub(crate) unsafe fn position_attribute_adjust(f: PlayerSheet, m: u32) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    if m & 0x1000 != 0 {
        (fb(player_sheet::FREE_ROLE) - 10) * 10
    } else if m & 0x80 != 0 {
        (fb(player_sheet::LEFT_SIDE) - 10) * 10
    } else if m & 0x100 != 0 {
        fb(player_sheet::LEFT_FOOT) + ((fb(player_sheet::LEFT_SIDE) / 2).max(fb(player_sheet::CENTRAL)) * 5 - 0x3c) * 2
    } else if m & 0x200 != 0 {
        (fb(player_sheet::CENTRAL) - 10) * 10
    } else if m & 0x400 != 0 {
        fb(player_sheet::RIGHT_FOOT) + ((fb(player_sheet::RIGHT_SIDE) / 2).max(fb(player_sheet::CENTRAL)) * 5 - 0x3c) * 2
    } else if m & 0x800 != 0 {
        (fb(player_sheet::RIGHT_SIDE) - 10) * 10
    } else {
        100
    }
}

/// Bonus from the recent-form exe::playerstats_get_recent_form(x0.2, weighted by days when the observer has known the player over a year) added
/// to the sum, scaled by `cont`.
pub(crate) unsafe fn recent_form_bonus(p: Staff, f: PlayerSheet, club: Club, who: Staff, count: i8) -> i32 {
    let mut x = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), f) as f64 * read_f64(rdata::F64_0_2)) as f32;
    if club == p.club_job() && !who.is_null() && scouted_over_a_year_ago(who, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64(rdata::F64_365) - d) * read_f64(rdata::F64_7_5) + d * x as f64) * read_f64(rdata::F64_0_002739726027)) as f32;
    }
    let x = x as f64;
    if x > read_f64(rdata::F64_6_8) {
        ftol((x * read_f64(rdata::F64_3_125) - read_f64(rdata::F64_21)) * count as f64) as i8 as i32
    } else if x > read_f64(rdata::F64_0) {
        ftol((x * read_f64(rdata::F64_6_25) - read_f64(rdata::F64_42)) * count as f64) as i8 as i32
    } else {
        0
    }
}

/// The form bonus (flag 2) added to the sum.
///
/// The rating `0x7c1e30(6)` capped by the average (or by `l3c`), limited for players over 33 when `teto_veterano`,
/// weighted by the days with observer `quem`, plus the ratings `0x7c1e30(1)` and `0x7c1e30(6, 0x14, `ultimo`)` at the
/// club (or the same nation).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn form_bonus(p: Staff, f: PlayerSheet, club: Club, who: Staff, age: i8, count: i8, sum: i32, l3c: i32, teto_veterano: bool, last: u32) -> i32 {
    let notes = read_usize(G_PLAYER_STATS);
    let ceiling = |v: i32| {
        let y = v as f64 * read_f32(rdata::F32_0_05000000075_AT_96D61C) as f64 + read_f32(rdata::F32_6_800000191) as f64;
        (if y > read_f64(rdata::F64_7_3) { read_f64(rdata::F64_7_3) } else { y }) as f32
    };
    let ratio = || {
        let (m10, m18) = (read_f64(notes + player_stats_fields::RATING_SUM), read_f64(notes + player_stats_fields::RATED_COUNT));
        if m18 == read_f64(rdata::F64_0) { read_f64(rdata::F64_6_8) } else { m10 / m18 }
    };
    let mut x = f32::from_bits(0x40d9999a);
    if count != 0 && sum / count as i32 > 0x96 {
        x = ceiling(sum / count as i32 - 0x96);
    } else if l3c as i16 > 0x96 {
        x = ceiling(l3c as i16 as i32 - 0x96);
    }
    let mut x = player_stat(f, 6, 0x12, x) as f32;
    if teto_veterano && age > 0x21 {
        let r = ratio() + read_f64(rdata::F64_0_125);
        if (x as f64) > r {
            x = r as f32;
        }
    }
    if club == p.club_job() && !who.is_null() && scouted_over_a_year_ago(who, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64(rdata::F64_365) - d) * read_f64(rdata::F64_7_5) + x as f64 * d) * read_f64(rdata::F64_0_002739726027)) as f32;
        let fb_p = signed_byte_at(p.player().addr(), 0xf);
        if age as i32 > fb_p / 10 + 0x22 && x as f64 > ratio() {
            x = ((ratio() + x as f64) * read_f32(rdata::F32_0_5) as f64) as f32;
        }
    }
    let xf = x as f64;
    let c = count as f64;
    let mut sum = sum;
    if xf > read_f64(rdata::F64_8) || (xf <= read_f64(rdata::F64_6_8) && xf > read_f64(rdata::F64_0)) {
        sum += ftol((xf * read_f32(rdata::F32_25) as f64 - read_f32(rdata::F32_170) as f64) * c) as i8 as i32;
    } else if xf > read_f64(rdata::F64_6_8) {
        sum += ftol((xf * read_f64(rdata::F64_12_5) - read_f64(rdata::F64_85)) * c) as i8 as i32;
    }
    let owner = p.club_job();
    if !club.is_null() && (club == owner || (!owner.is_null() && club.nation() == owner.nation())) {
        let xk = (xf * read_f64(rdata::F64_0_9)) as f32;
        let r = player_stat(f, 1, 0x12, xk);
        if r > read_f64(rdata::F64_6_8) {
            sum += ftol((r * read_f64(rdata::F64_6_25) - read_f64(rdata::F64_43)) * c) as i8 as i32;
        } else if r < xk as f64 {
            sum += ftol((r * read_f32(rdata::F32_85) as f64 / xk as f64 - read_f32(rdata::F32_85) as f64) * c) as i8 as i32;
        }
        sum += ftol((player_stat(f, 6, 0x14, f32::from_bits(last)) * read_f64(rdata::F64_40) - read_f64(rdata::F64_60)) * c);
    }
    sum
}

/// Penalty from the player's injury record (+0x28) subtracted from l4c.
pub(crate) unsafe fn injury_history_penalty(p: Staff, l4c: i32) -> i32 {
    let h = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), p));
    if h.is_null() {
        return l4c;
    }
    let w = h.days_out() as i32;
    l4c + if w <= 10 { -2 * w } else { -20 - w }
}

/// Value from INJURY `exe::injury_get_fatigue` by person id (exe map: get_fatigue / word_0c; the code comment calls it reputation)
/// [inference].
pub(crate) unsafe fn injury_word_0c(p: usize) -> i16 {
    exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p))
}

/// End of flag 0x4000: age penalty (flag 0x100), half-attribute sum bonus, +0x2f and the foreign player's
/// language/continent factor.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn age_and_foreigner_adjust(p: Staff, f: PlayerSheet, club: Club, flags: u32, pos: u32, masc: u32, age: i8, count: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    if flags & 0x100 != 0 {
        let cl = if masc & 1 != 0 { 0x1d } else { 0x19 };
        let y = read_f64(rdata::F64_3_5) - count as f64 * read_f64(rdata::F64_0_67);
        if age as i32 > cl && y > read_f64(rdata::F64_0) {
            l4c -= ftol((age as i32 - cl) as f64 * y);
        }
    }
    if pos & 1 == 0 {
        let s: i32 = (0x10..=0x19).map(|k| fb(k) / 2).sum();
        if s > 0x23 {
            l4c += (s - 20) * (s - 20) / 250;
        }
    }
    l4c += ftol((fb(player_sheet::LEADERSHIP) - 10) as f64 * read_f64(rdata::F64_0_6666666667));
    let nation = club.nation();
    let pais = p.nation();
    if nation.is_null() || (nation.first_language() as u8) == (pais.first_language() as u8) {
        return l4c;
    }
    let mut c = (exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, (nation.addr() as u32 & !0xff) | (nation.first_language() as u8) as u32, 1) as u32) as u8 as i8;
    if flags & 0x100 != 0 {
        let k = (age as u8).wrapping_sub((p.adaptability() / 2) as u8).wrapping_add(5) as i8;
        let m = read_f64(rdata::F64_26) - k as f64 * read_f64(rdata::F64_0_83);
        c = ftol(if c as f64 > m { c as f64 } else { m }) as i8;
    } else {
        let y = l4c as f64 * read_f64(rdata::F64_0_06) - read_f64(rdata::F64_5);
        if (c as f64) < y {
            c = ftol(y) as i8;
        }
    }
    let b22 = p.int_apps() as i32;
    if b22 != 0 && b22 * b22 + pais.reputation() as i32 > 5000 && c <= 5 {
        c = 5;
    }
    if (0..=9).contains(&c) {
        const IGUAL: [usize; 10] = [0x96a6a8, 0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8];
        const OUTRO: [usize; 10] = [0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8, 0x968e50];
        let t = if pais.actual_region() == world_region::UK_AND_IRELAND { &IGUAL } else { &OUTRO };
        l4c = ftol(l4c as f64 * read_f64(t[c as usize]));
    }
    l4c
}

/// Adjustment by contract (flag 0x2000000): days without club, wage vs owner and satisfaction, age, goalkeeper, injury
/// and appearances.
pub(crate) unsafe fn contract_adjust(p: Staff, f: PlayerSheet, club: Club, contract: usize, age: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let mul = |l: i32, k: f64| ftol(l as f64 * k);
    let owner = p.club_job();
    if owner.is_null() {
        if exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32 , p.date_joined_club_leap_year() as u32) > 0x3c {
            l4c = mul(l4c, read_f64(rdata::F64_0_75));
        }
    } else {
        let lim = owner.reputation() as i32 - 0x4e2;
        let dx = club.reputation();
        let ax = read_i8(contract + contract::HAPPINESS) as i16;
        let cl = age;
        let k = if dx as i32 > lim {
            let ax = ax.wrapping_add(0x64);
            if ax >= 0x7d {
                None
            } else {
                let mut st = read_f32(rdata::F32_0_75) as f64;
                if dx != 0 {
                    st = lim as f64 / dx as f64 * ax as f64 * read_f32(rdata::F32_0_00800000038) as f64;
                    if st > read_f64(rdata::F64_1) {
                        st = read_f32(rdata::F32_1) as f64;
                    } else if st < read_f64(rdata::F64_0_75) {
                        st = read_f32(rdata::F32_0_75) as f64;
                    }
                }
                if cl > 0x22 {
                    st *= read_f64(rdata::F64_0_75);
                } else if cl > 0x1e {
                    st *= read_f64(rdata::F64_0_85);
                } else if cl > 0x1b {
                    st *= read_f64(rdata::F64_0_95);
                }
                if fb(player_sheet::GOALKEEPER) == 0x14 {
                    st *= read_f64(if cl < 0x12 { 0x969040 } else if cl < 0x14 { 0x969520 } else { 0x9689d0 });
                }
                Some(st)
            }
        } else if ax >= 0x19 || cl <= 0x1e {
            None
        } else {
            let mut st = read_f32(if cl > 0x25 { 0x968f18 } else if cl > 0x22 { 0x96d248 } else { 0x96d364 }) as f64;
            if dx > 0x1482 && cl < 0x14 {
                if ax < -0x4b {
                    st = read_f32(rdata::F32_0_5) as f64;
                } else if ax < -0x19 {
                    st = read_f32(rdata::F32_0_6499999762) as f64;
                } else if ax < 0 {
                    st = read_f32(rdata::F32_0_8500000238) as f64;
                }
            }
            if fb(player_sheet::GOALKEEPER) == 0x14 {
                st *= read_f64(if cl < 0x12 { 0x969040 } else if cl < 0x15 { 0x969520 } else { 0x9689d0 });
            }
            Some(st)
        };
        if let Some(st) = k {
            l4c = mul(l4c, st);
        }
        if exe::staff_is_transfer_listed(p, (0) as usize) != 0 {
            l4c = mul(l4c, read_f64(rdata::F64_0_95));
        }
    }
    let (cl, al) = (age, p.int_apps());
    // Some(0): a nota vira 5
    let factor: Option<usize> = if cl > 0x22 {
        if al == 0 {
            Some(0x9689e0)
        } else if al < 8 {
            Some(0x968968)
        } else if cl > 0x23 && al < 0x19 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x969040)
        } else if al < 0x23 {
            Some(0x969520)
        } else {
            None
        }
    } else if cl > 0x1b {
        if al == 0 {
            Some(0x968968)
        } else if al < 8 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x9689b0)
        } else {
            None
        }
    } else if cl > 0x18 {
        if al == 0 {
            Some(0x96a5d0)
        } else if al < 8 {
            Some(0x969040)
        } else if al < 0xf {
            Some(0x969520)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if cl > 0x15 {
        if al == 0 {
            Some(0x969520)
        } else if al < 6 {
            Some(0x9689d8)
        } else if al < 0xa {
            Some(0x9689d0)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if al > 0xa {
        Some(0)
    } else {
        None
    };
    match factor {
        Some(0) => 5,
        Some(k) => mul(l4c, read_f64(k)),
        None => l4c,
    }
}

/// The observer bonus (flag 0x1000): the knowledge `conhece` (+0x22 for the youngsters, +0x14, +0x0e, +0x1a, +0x1f,
/// +0x13), the player's liking for the club and for `quem`, and the attributes of each position band.
///
/// `craque_meia` also turns on the star players' bonus in band 0x20 (only in `off_match_score`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn scout_knowledge_bonus(p: Staff, f: PlayerSheet, club: Club, conhece: usize, obs: usize, who: Staff, pos: u32, masc: u32, age: i8, mut l4c: i32, craque_meia: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let kb = |k: usize| signed_byte_at(conhece, k);
    if age <= 0x17 {
        l4c += ftol((kb(0x22) - 10) as f64 * read_f64(rdata::F64_0_6666666667));
    }
    let g = |a: usize| -> unsafe extern "C" fn(usize, usize) -> u32 { core::mem::transmute(a) };
    if g(0x547d30)(p.addr(), club.addr()) != 0 {
        let m = l4c as f64 * read_f64(rdata::F64_0_75);
        let x = (l4c - 10) as f64;
        l4c = ftol(if x < m { x } else { m });
    } else if g(0x547d60)(p.addr(), club.addr()) != 0 {
        let m = l4c as f64 * read_f64(rdata::F64_1_1);
        let x = (l4c + 10) as f64;
        l4c = ftol(if x > m { x } else { m });
    }
    if g(0x547840)(p.addr(), who.addr()) != 0 {
        l4c -= 50;
    } else if g(0x547810)(p.addr(), who.addr()) != 0 {
        let x = l4c as f64 * read_f64(rdata::F64_1_25);
        let m = (l4c + 20) as f64;
        l4c = ftol(if x > m { x } else { m });
    }
    let f44 = fb(player_sheet::WORK_RATE);
    let k14 = kb(0x14);
    let mut ecx = (k14 * (f44 - 10) / 3).min(10) + ((signed_byte_at(p.addr(), 0x5b) - 10) * k14 / 3).min(10) + ((signed_byte_at(p.addr(), 0x59) - 10) * signed_byte_at(obs, 0x59) / 3).min(10);
    if pos & 1 == 0 {
        let away = pos & 0x80 == 0 && pos & 0x800 == 0;
        let e = kb(0xe);
        let k1a = kb(0x1a) * fb(player_sheet::MARKING);
        let k1f = kb(0x1f) * (f44 - 10);
        // the stars: (+0x26 / 3) x +0x1b x +0x36 / 1500, with +0x26 > 30 and +0x36 > 15
        let craque = |ecx: i32| if fb(player_sheet::DRIBBLING) > 0x1e && fb(player_sheet::PLAYER_PACE) > 0xf { ecx + fb(player_sheet::DRIBBLING) / 3 * fb(player_sheet::ACCELERATION) * fb(player_sheet::PLAYER_PACE) / 1500 } else { ecx };
        if pos & 6 != 0 {
            ecx += k1a / 40 + (fb(player_sheet::FLAIR) - 10) * e / 10 + fb(player_sheet::DRIBBLING) * e / 500 + fb(player_sheet::FINISHING) * e / 350 + fb(player_sheet::VISION) * e / 250 + k1f / 8 - 10;
            if away {
                ecx += fb(player_sheet::JUMPING) - 15;
            }
        } else if pos & 8 != 0 {
            ecx += k1a / 40 + (fb(player_sheet::FLAIR) - 10) * e / 8 + fb(player_sheet::DRIBBLING) * e / 325 + fb(player_sheet::VISION) * e / 150 + k1f / 6 + fb(player_sheet::FINISHING) / 15 - 10;
            if away {
                ecx += fb(player_sheet::TACKLING) / 15 + f44 - 15;
            }
        } else if pos & 0x10 != 0 {
            ecx += k1a / 60 + (fb(player_sheet::FLAIR) - 10) * e / 5 + fb(player_sheet::DRIBBLING) * e / 150 + fb(player_sheet::VISION) * e / 100 + k1f / 6 + fb(player_sheet::FINISHING) / 10 - 10;
            if away {
                ecx += fb(player_sheet::TACKLING) / 15 + fb(player_sheet::PASSING) / 15 + f44 - 15;
            } else {
                ecx += fb(player_sheet::CROSSING) / 15;
            }
        } else if pos & 0x20 != 0 {
            ecx += k1a / 150 + (fb(player_sheet::FLAIR) - 10) * e / 4 + k1f / 8 + fb(player_sheet::VISION) * e / 75 + fb(player_sheet::DRIBBLING) * e / 150 + fb(player_sheet::FINISHING) / 8 - 10;
            if away {
                ecx += fb(player_sheet::VISION) / 15 + fb(player_sheet::FREE_ROLE) - 15;
            } else {
                ecx += fb(player_sheet::CROSSING) / 15;
            }
            if craque_meia {
                ecx = craque(ecx);
            }
        } else if pos & 0x40 != 0 {
            ecx += k1a / 400 + (fb(player_sheet::FLAIR) - 10) * e / 3 + fb(player_sheet::VISION) * e / 200 + k1f / 10 + fb(player_sheet::DRIBBLING) * e / 175 + fb(player_sheet::FINISHING) / 5 - 10;
            if away {
                ecx += fb(player_sheet::MOVEMENT) / 15 + fb(player_sheet::PLAYER_PACE) + fb(player_sheet::TECHNIQUE) + fb(player_sheet::FINISHING) / 15 - 0x14;
                ecx = craque(ecx);
            }
        }
    }
    if masc & 0x40 != 0 && pos & 0x40 != 0 {
        ecx += (fb(player_sheet::PLAYER_PACE) * fb(player_sheet::JUMPING) - 200) * kb(0x13) / 500;
    }
    if ecx != 0 {
        l4c += ftol((read_f64(rdata::F64_1) - l4c as f64 * read_f64(rdata::F64_0_00375)) * ecx as f64).clamp(-0x50, 0x50);
    }
    l4c
}

/// If the player has a work-permit record and needs a permit at the contract club: percentage +4/(+4 + +6) as a byte (0
/// if none); None otherwise or at 70+.
pub(crate) unsafe fn work_permit_appearance_pct(p: Staff) -> Option<i8> {
    let list = read_usize(G_TRANSFER_MANAGER);
    let x = exe::transfer_manager_find_work_permit(list, p);
    if x == 0 {
        return None;
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let owner = if c == 0 { 0 } else { read_usize(G_CLUB_TABLE) + read_usize(c + 4) * club::SIZE };
    if exe::transfer_manager_player_needs_work_permit(list, p, owner, 1) == 0 {
        return None;
    }
    let (a, b) = (read_u16(x + 4) as i32, read_u16(x + 6) as i32);
    if a + b == 0 {
        return Some(0);
    }
    let bl = (a * 100 / (a + b)) as i8;
    if bl >= 0x46 { None } else { Some(bl) }
}

/// Contract extra (flag 0x10000): +0x44 by the +0x45 flags, the squad status (+0x4f >> 4) and the work-permit
/// percentage.
pub(crate) unsafe fn contract_extra(p: Staff, contract: Contract, flags: u32, l28: i32) -> i32 {
    let e = contract.grievances();
    let s44 = contract.morale() as f64;
    let mut extra = 0i32;
    if e & 0x2808 != 0 {
        let x = s44 * read_f64(rdata::F64_0_6666666667);
        extra = ftol(if read_f64(rdata::F64_NEG_25) < x { read_f64(rdata::F64_NEG_25) } else { x });
    } else if e != 0 {
        let x = s44 * read_f64(rdata::F64_0_4);
        extra = ftol(if read_f64(rdata::F64_NEG_5) < x { read_f64(rdata::F64_NEG_5) } else { x });
    }
    let t = (contract.squad_status() >> 4) as i8 as i32 - 1;
    let list = match t {
        0 | 1 => {
            extra += if t == 0 { 0x19 } else { 0xf };
            flags & 0x800000 == 0
        }
        2 => {
            if flags & 0x800000 != 0 {
                extra += 10;
                false
            } else {
                let d = (contract.unhappy_flag() as i32 - (contract.matches_left_out() as i8) as i32) * 5;
                extra += ftol(d as f64 + (l28 as i16 as f64 * read_f64(rdata::F64_2_5) - read_f64(rdata::F64_225)));
                true
            }
        }
        6 => {
            extra -= 0x14;
            false
        }
        _ => false,
    };
    if list {
        if let Some(bl) = work_permit_appearance_pct(p) {
            extra += 0x32 - bl as i32 / 2;
        }
    }
    extra
}

/// Bond (flag 0x20000) by the contract's squad status: -25 to +25, or by the work-permit percentage.
pub(crate) unsafe fn contract_bond(p: Staff, c: Contract, flags: u32) -> i32 {
    let t = (c.squad_status() >> 4) as i8 as i32 - 1;
    match t {
        0..=2 => {
            if flags & 0x800000 != 0 {
                -25
            } else {
                match work_permit_appearance_pct(p) {
                    Some(bl) => bl as i32 / 2 - 0x4b,
                    None => -25,
                }
            }
        }
        3 | 4 | 7 => 25,
        _ => 0,
    }
}

/// Adjusts l4c when the `exe::injury_get_fatigue` value is below -1000 or above 500.
pub(crate) unsafe fn injury_word_0c_adjust(p: Staff, l4c: i32) -> i32 {
    if injury_word_0c(p.addr()) < -1000 {
        l4c - ftol(injury_word_0c(p.addr()) as f64 * read_f64(rdata::F64_0_0125) + read_f64(rdata::F64_12_5))
    } else if injury_word_0c(p.addr()) > 0x1f4 {
        l4c + ftol(read_f64(rdata::F64_25) - injury_word_0c(p.addr()) as f64 * read_f64(rdata::F64_0_05))
    } else {
        l4c
    }
}

/// Appearances penalty (+0x22, flag 0x100000) or club adjustment (flag 4: owner +0x39 and their manager's +0x69).
pub(crate) unsafe fn appearances_or_club_adjust(p: Staff, f: PlayerSheet, flags: u32, of_the_base: bool, l4c: i32) -> i32 {
    if flags & 0x100000 != 0 {
        return l4c - (p.int_apps() as i32 * 5).min(0x32);
    }
    if flags & 4 == 0 || !of_the_base {
        return l4c;
    }
    let owner = p.club_job();
    if owner.is_null() {
        return if f.home_reputation() < 0x1388 { l4c - 0x1e } else { l4c };
    }
    let s = owner.division();
    if s.is_null() {
        return l4c;
    }
    l4c + ftol((s.reputation() as i32 - 0x10) as f64 * read_f64(rdata::F64_5))
}

/// At the player's own club (+0x39): rating up by 5 or 5%, whichever is larger.
pub(crate) unsafe fn own_club_bonus(p: Staff, club: Club, l4c: i32) -> i32 {
    if p.club_job() != club {
        return l4c;
    }
    let x = (l4c + 5) as f64;
    let m = l4c as f64 * read_f64(rdata::F64_1_05);
    ftol(if x > m { x } else { m })
}

/// Combines the position rating l40 with the adjustment l4c (only l40 when flags == 0x800).
pub(crate) fn combine_rating(flags: u32, l40: i32, l4c: i32) -> i32 {
    if flags == 0x800 {
        l40
    } else if l40 < 0 {
        (500 - l4c) * l40
    } else if l4c < 1 {
        l4c + l40
    } else {
        l40 * l4c
    }
}

/// The player's score for a position outside a match (transfers and scouting): position, form, perceived ability,
/// contract, age, scout knowledge.
///
/// The score of player `p` for position `pos` outside the match (transfers, scouting), by the `flags`, for `clube`,
/// with the cut-off `limite` and scout `o`: the position score (table `0xa15d50`) times the adjustment for reputation,
/// form, perceived ability, age, contract and the scout's knowledge. -10000 with no player sheet (`+0x61`) or below the
/// cut-off.
/// Original: `off_match_score` (0x6e6340, match_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn off_match_score(p: usize, pos: u32, flags: u32, club: usize, limit: i32, o: usize) -> i32 {
    let p = Staff(p);
    let o = Staff(o);
    let club = Club(club);
    if p.is_null() {
        return -10000;
    }
    let f = p.player();
    if f.is_null() {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let bd = read_usize(G_INJURY);
    let of_the_base = read_i32(p.addr()) < read_i32(G_STAFF_COUNT);
    let active = limit > -1;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    let without_value = flags & 0x800;
    if without_value == 0 || flags & 0x210000 != 0 {
        l24 = exe::injury_get_condition(bd, p, 0) as i32;
        l28 = l24 as i16 as i32 / 100;
    }
    let age = p.age() as i8;
    let contract = if read_usize(G_CONTRACT_MANAGER) != 0 { exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club) } else { 0 };
    let (conhece, obs) = if !o.is_null() {
        (o.non_player().addr(), o.addr())
    } else if !club.is_null() && club.manager().addr() != 0 {
        let t = club.manager();
        (t.non_player().addr(), t.addr())
    } else {
        (0, 0)
    };
    let mut l3c = exe::database_get_staff_reputation_for_viewer(p, obs, 0) as i32;
    let masc = exe::formation_get_position_mask(f) as u32;
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    edi += if flags & 0x400 != 0 { 100 } else { position_rating(f, pos, masc, 0xa15d50, true) };
    let l40 = edi + if flags & 0x200 != 0 { 100 } else { position_attribute_adjust(f, pos) };
    let mut count: i8 = 0;
    let mut sum: i32 = 0;
    let cut = |sum: i32, count: i8| active && sum * 200 / (count as i32) < limit;
    if flags & 0x40 != 0 {
        count = 1;
        sum = f.current_reputation() as i32 / 50;
        if club == p.club_job() && !o.is_null() && scouted_over_a_year_ago(o, p) {
            sum = ftol((read_f64(rdata::F64_1_375) - match_man_days_since_date(p) as f64 * read_f64(rdata::F64_0_001)) * sum as f64);
        }
        if active && sum * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        count += 1;
        let a = f.home_reputation() as f64;
        let b = p.nation().reputation() as f64;
        sum += ftol(a / ((read_f64(rdata::F64_1_8) - b * read_f64(rdata::F64_0_0001)) * read_f64(rdata::F64_50)));
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        count += 1;
        sum += f.world_reputation() as i32 / 50;
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 4 != 0 && of_the_base {
        let v = ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64(rdata::F64_20));
        if v != 0 {
            count += 1;
            sum += v;
        }
    }
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if count != 0 {
            l3c = sum * 50 / count as i32;
        }
        l34 = exe::match_man_judge_player_ability(obs, p, club, 2, l3c as i16) as u16 as i32;
        count += 1;
        sum += l34 as i16 as i32;
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if count != 0 {
            l3c = potential_or_average_value(p, f, club, sum, count);
        }
        let mut r = exe::match_man_judge_player_ability(obs, p, club, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        count += 1;
        sum += r as i32;
        if cut(sum, count) {
            return -10000;
        }
    }
    // from here on the original reuses the limit’s slot for calculations: it no longer cuts
    if flags & 0x40000 != 0 && of_the_base {
        sum += recent_form_bonus(p, f, club, o, count);
    }
    if flags & 2 != 0 && of_the_base {
        sum = form_bonus(p, f, club, o, age, count, sum, l3c, false, 0x3fb66666);
    }
    let mut l4c: i32 = 0;
    if count != 0 {
        l4c = sum / count as i32;
        if l4c > 0x96 {
            l4c = ftol((l4c - 0x96) as f64 * read_f64(rdata::F64_1_5) + read_f64(rdata::F64_150));
        }
    }
    if flags & 8 != 0 && of_the_base {
        l4c += ftol(fb(player_sheet::PLAYER_MORALE) as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_15));
    }
    if flags & 0x4000 != 0 {
        let q = (0x1b58 - club.reputation() as i32) / 100;
        let k = (if q < 10 { 10 } else { q }) as i8 as i32;
        let lim = if fb(player_sheet::GOALKEEPER) == 0x14 { 0x24 } else if fb(player_sheet::ATTACKER) == 0x14 || fb(player_sheet::ATTACKING_MIDFIELDER) == 0x14 { 0x22 } else { 0x23 };
        if age as i32 > lim {
            let y = (read_f64(rdata::F64_1) - (age as i32 - lim) as f64 / k as f64) * l4c as f64;
            let m = (l4c - 5) as f64;
            l4c = ftol(if y < m { y } else { m });
        }
        let h = InjuryEntry(exe::injury_get_player_injury(bd, p));
        if !h.is_null() {
            l4c -= h.days_left() as i32;
        }
        l4c = age_and_foreigner_adjust(p, f, club, flags, pos, masc, age, count, l4c);
    }
    if contract != 0 && !club.is_null() && flags & 0x2000000 != 0 && of_the_base {
        l4c = contract_adjust(p, f, club, contract, age, l4c);
    }
    if without_value == 0 {
        let mut v = l24 as i16 as i32;
        if flags & 0x200000 == 0 && injury_word_0c(p.addr()) > 1000 {
            v += 500 - injury_word_0c(p.addr()) as i32;
        }
        if v < 5000 {
            return v + (l40 * 5 - 10000) * 20;
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && !club.is_null() && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, club, conhece, obs, o, pos, masc, age, l4c, true);
    }
    l4c = outside_nation_penalty(p, club, flags, l4c);
    if flags & 0x10000 != 0 {
        if contract != 0 {
            l4c += contract_extra(p, Contract(contract), flags, l28);
        }
    } else if flags & 0x20000 != 0 {
        let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        if c != 0 {
            l4c += contract_bond(p, Contract(c), flags);
        }
        if without_value == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, of_the_base, l4c);
    if !of_the_base {
        l4c -= 0x1e;
    }
    l4c = own_club_bonus(p, club, l4c);
    combine_rating(flags, l40, l4c)
}

/// Value for potential ability (flag 0x100): at the contract club, average x25 plus potential (+7) x25 (16-bit);
/// elsewhere average x50.
pub(crate) unsafe fn potential_or_average_value(p: Staff, f: PlayerSheet, club: Club, sum: i32, count: i8) -> i32 {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let owner = if c == 0 { 0 } else { read_usize(G_CLUB_TABLE) + read_usize(c + 4) * club::SIZE };
    if owner == club.addr() {
        let q = sum * 25 / count as i32;
        let _ = f.addr();
        let pot = read_i16(p.player().addr() + 7).wrapping_mul(0x19) as u16 as u32;
        q.wrapping_add(((q as u32 & 0xffff0000) | pot) as i32)
    } else {
        sum * 50 / count as i32
    }
}

/// Fame (flag 0x200000): -200 for famous players (value/100 >= 99) with high +0x3c, low +0x2d and `exe::injury_get_fatigue` value < 500;
/// else 100 - value/100.
pub(crate) unsafe fn fame_adjust(f: PlayerSheet, p: Staff, l28: i32, l4c: i32) -> i32 {
    let v = l28 as i16 as i32;
    if v >= 0x63 && signed_byte_at(f.addr(), 0x3c) > 0xe && signed_byte_at(f.addr(), 0x2d) < 8 && injury_word_0c(p.addr()) < 0x1f4 {
        l4c - 200
    } else {
        l4c + 100 - v
    }
}

/// Flag 0x1000000: -50 if neither nationality (+0x1a, +0x1e) is the club's nation.
pub(crate) unsafe fn outside_nation_penalty(p: Staff, club: Club, flags: u32, l4c: i32) -> i32 {
    if flags & 0x1000000 == 0 || club.is_null() {
        return l4c;
    }
    let nation = club.nation().addr();
    if p.nation() != Nation(nation) && p.second_nation() != Nation(nation) {
        l4c - 50
    } else {
        l4c
    }
}

/// The player's score for a lineup slot: like the off-match score, plus the second position, in-match condition,
/// tactics, veterans and holidays.
///
/// The score of player `p` (player sheet `f`) for a lineup slot (`pos`, and the second position `pos2` with weight
/// 1/3), by the `flags`, at `clube`, with side `lado` (`+0x1b` scout, `+0x1f` knowledge), manager `tecnico`, the
/// `limite` (cut-off), the tactic flags (`marca` and `bytes`, `tatica`) and the holiday list `ferias` (16 pairs of id,
/// position). Like `0x6e6340` (table `0xa15cfc`), plus the condition in the match (`reg`, the player in the match), the
/// tactic, the veterans' age and the holiday bonus. -10000 with no person or player sheet; -1000000 if vetoed.
/// Original: `lineup_slot_score` (0x6e2f20, match_man.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, club: usize, side: usize, manager: usize, limit: i32, mark: u32, _marca_alta: u32,
    bytes: u32, tactic: u32, pos2: u32, holiday: usize,
) -> i32 {
    let side = MatchManager(side);
    let f = PlayerSheet(f);
    let reg = MatchPlayer(reg);
    let p = Staff(p);
    let manager = Staff(manager);
    let club = Club(club);
    if p.is_null() || f.is_null() {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let bd = *(G_INJURY as *const usize);
    let of_the_base = *(p.addr() as *const i32) < *(G_STAFF_COUNT as *const i32);
    let age = p.age() as i8;
    let contract = if *(G_CONTRACT_MANAGER as *const usize) != 0 { exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), p, club) } else { 0 };
    let without_value = flags & 0x800;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    if without_value == 0 || flags & 0x210000 != 0 {
        l24 = exe::injury_get_condition(bd, p, 0) as i32;
        l28 = l24 as i16 as i32 / 100;
    }
    let limit = if holiday != 0 { 0 } else { limit };
    let active = holiday != 0 || limit > -1;
    let (conhece, obs) = if !side.is_null() {
        (side.manager_sheet().addr(), side.manager())
    } else if !manager.is_null() {
        (manager.non_player().addr(), manager.addr())
    } else if !club.is_null() && club.manager().addr() != 0 {
        let t = club.manager();
        (t.non_player().addr(), t.addr())
    } else {
        (0, 0)
    };
    let mut l3c = exe::database_get_staff_reputation_for_viewer(p, obs, 0) as i32;
    let masc = exe::formation_get_position_mask(f) as u32;
    let second = pos2 as u16 != 0 && pos2 as u16 != pos as u16;
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    let mut ebp: i32 = 0;
    if flags & 0x400 != 0 {
        edi += 100;
    } else {
        edi += position_rating(f, pos, masc, 0xa15cfc, true);
        if second {
            ebp = position_rating(f, pos2, masc, 0xa15cfc, false);
        }
    }
    let l40 = if flags & 0x200 != 0 {
        edi + 100
    } else {
        edi += position_attribute_adjust(f, pos);
        if second { (ebp + position_attribute_adjust(f, pos2) + 2 * edi) / 3 } else { edi }
    };
    let mut count: i8 = 0;
    let mut sum: i32 = 0;
    let cut = |sum: i32, count: i8| active && sum * 200 / (count as i32) < limit;
    if flags & 0x40 != 0 {
        count = 1;
        sum = f.current_reputation() as i32 / 50;
        if club == p.club_job() && !manager.is_null() && scouted_over_a_year_ago(manager, p) {
            sum = ftol((read_f64(rdata::F64_1_375) - match_man_days_since_date(p) as f64 * read_f64(rdata::F64_0_001)) * sum as f64);
        }
        if active && sum * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        count += 1;
        let a = f.home_reputation() as f64;
        let b = core::ptr::read_unaligned((p.nation().addr() + nation::REPUTATION) as *const i16) as f64;
        sum += ftol(a / ((read_f64(rdata::F64_2) - b * read_f64(rdata::F64_0_0001)) * read_f64(rdata::F64_50)));
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        count += 1;
        sum += f.world_reputation() as i32 / 50;
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 1 != 0 && !reg.is_null() && masc & 1 == 0 {
        count += 1;
        sum += (reg.match_rating() as i32 - match_fatigue_offset(reg, fb(player_sheet::GOALKEEPER))) / 50;
    }
    if flags & 4 != 0 && of_the_base {
        let v = ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64(rdata::F64_20));
        if v != 0 {
            count += 1;
            sum += v;
        }
    }
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if count != 0 {
            l3c = sum * 50 / count as i32;
        }
        l34 = exe::match_man_judge_player_ability(obs, p, club, 2, l3c as i16) as u16 as i32;
        count += 1;
        sum += l34 as i16 as i32;
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if count != 0 {
            l3c = potential_or_average_value(p, f, club, sum, count);
        }
        let mut r = exe::match_man_judge_player_ability(obs, p, club, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        let fp = p.player();
        if !fp.is_null() {
            let (d, c) = (fp.acceleration(), fp.player_pace());
            r = r.wrapping_sub(if d < 6 { 10 } else if d < 10 { 5 } else { 0 });
            r = r.wrapping_sub(if c < 6 { 15 } else if c < 10 { 10 } else { 0 });
        }
        count += 1;
        sum += r as i32;
        if cut(sum, count) {
            return -10000;
        }
    }
    if flags & 0x40000 != 0 && of_the_base {
        sum += recent_form_bonus(p, f, club, manager, count);
    }
    if flags & 2 != 0 && of_the_base {
        sum = form_bonus(p, f, club, manager, age, count, sum, l3c, true, 0x3faccccd);
    }
    let mut l4c: i32 = 0;
    if count != 0 {
        l4c = sum / count as i32;
        if l4c > 0x96 {
            l4c = ftol((l4c - 0x96) as f64 * read_f64(rdata::F64_1_5) + read_f64(rdata::F64_150));
        }
    }
    if flags & 8 != 0 && of_the_base {
        l4c += ftol((fb(player_sheet::PLAYER_MORALE) - 10) as f64 * read_f64(rdata::F64_0_8));
    }
    let marked = mark & 0x8000 != 0 || (mark & 0x2000 != 0 && tactic & 0x100 != 0);
    if flags & 0x8000 != 0 {
        l4c = tactic_instruction_adjust(f, mark, bytes as u8, tactic, marked, l4c);
    }
    if contract != 0 && !club.is_null() && flags & 0x2000000 != 0 && of_the_base {
        l4c = contract_adjust(p, f, club, contract, age, l4c);
    }
    if flags & 0x4000 != 0 {
        l4c = injury_history_penalty_2c(p, l4c);
        let id = *(p.addr() as *const i32);
        let mut edi = l4c;
        if *((*(G_STAFF_STATE_TABLE as *const usize) as isize + id as isize * (staff_state::SIZE as isize) + (staff_state::STATUS_FLAGS as isize)) as *const u8) & staff_state_status_flags::RETIREMENT_STATE != 0 {
            if age > 0x28 {
                return -1000000;
            }
            edi -= if age > 0x22 { 0x4b } else { 0x19 };
            l4c = edi;
            if signed_byte_at(p.player().addr(), 0x3c) < 0xa {
                edi -= 0x32;
                l4c = edi;
            }
        }
        if age > 0x2a {
            return -1000000;
        }
        let i = age as i32;
        if i > 0x22 && masc & 1 == 0 {
            l4c = edi + (0x22 - i) * 10;
        } else if i < 0x17 && masc & 1 != 0 {
            l4c = edi + (i - 0x17) * 10;
        } else if i > 0x20 && pos & 0x40 != 0 {
            l4c = edi + (0x20 - i) * 5;
        }
        l4c = age_and_foreigner_adjust(p, f, club, flags, pos, masc, age, count, l4c);
    }
    let mut value: i32 = 0;
    if without_value == 0 {
        let mut v: i32;
        if !reg.is_null() {
            if reg.position() == match_slot::INJURED_OFF {
                return -1000000;
            }
            let x = reg.stamina_boost() as f64;
            v = ftol(x + x + reg.stamina() as f64) as i16 as i32;
            if core::ptr::read_unaligned((reg.engine().addr() + engine::PERIOD_END) as *const i16) == 0x1ef {
                v = (v + 0x5dc).min(0x2710);
            }
        } else {
            v = l24 as i16 as i32;
        }
        if flags & 0x200000 == 0 && injury_word_0c(p.addr()) > 1000 {
            v += 500 - injury_word_0c(p.addr()) as i32;
        }
        if v < 0x1676 {
            let r = v + (l40 * 5 - 0x4e2) * 20;
            return if v >= 0xdac { r } else { r - 0x7a120 };
        }
        if marked {
            v = ftol(v as f64 * read_f64(rdata::F64_0_0001) * v as f64);
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
        value = v;
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && !club.is_null() && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, club, conhece, obs, manager, pos, masc, age, l4c, false);
    }
    if flags & 0x2000 != 0 && !side.is_null() && side.best_player() == p.addr() {
        l4c += 100;
    }
    l4c = outside_nation_penalty(p, club, flags, l4c);
    if flags & 0x80000 != 0 && !side.is_null() {
        let em = |o: usize| *((side.addr() + o) as *const usize) == p.addr();
        l4c += if em(match_manager::KEY_PLAYER_1) { 0x1e } else if em(match_manager::KEY_PLAYER_2) { 0x14 } else if em(match_manager::KEY_PLAYER_3) { 0xa } else { 0 };
    }
    // fatigue at the match minute reduces the extra
    let by_minute = |v: i32| {
        if reg.is_null() {
            v
        } else {
            let minute = core::ptr::read_unaligned((reg.engine().addr() + engine::CLOCK) as *const i16) as f64;
            ftol((read_f64(rdata::F64_1) - minute * read_f64(rdata::F64_0_0008333333333)) * v as f64)
        }
    };
    if flags & 0x10000 != 0 {
        if contract != 0 {
            let mut extra = by_minute(contract_extra(p, Contract(contract), flags, l28));
            if holiday != 0 {
                extra = ftol(extra as f64 * read_f64(rdata::F64_0_25));
            }
            l4c += extra;
        }
    } else if flags & 0x20000 != 0 {
        if contract != 0 {
            l4c += by_minute(contract_bond(p, Contract(contract), flags));
        }
        if without_value == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, of_the_base, l4c);
    let mut base = l40;
    if !of_the_base && !club.is_null() {
        let nation = club.nation();
        if !nation.is_null() && nation.league_selected() != 0 {
            l4c -= 0x46;
            base = l40 - 0x78;
        }
    }
    l4c = own_club_bonus(p, club, l4c);
    l4c = combine_rating(flags, base, l4c);
    if holiday != 0 && (pos & 1 == 0 || masc & 1 != 0) {
        l4c = holiday_selection_bonus(p, club, holiday, pos, base, without_value, value, l4c);
    }
    l4c
}

/// Flag 0x4000 penalty from the injury record (+0x2c).
pub(crate) unsafe fn injury_history_penalty_2c(p: Staff, l4c: i32) -> i32 {
    let h = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), p));
    if h.is_null() { l4c } else { l4c - h.days_left() as i32 }
}

/// The fatigue of match player `reg` to subtract from stamina (`+0x35`).
///
/// −2000 for the human manager (`+0x904d` of the side, without `0x5fd210`, national team or `0x5fd480`) with the player
/// below 11 (`+0`); off the pitch (`+0x19` = −1) the remaining minutes × 150 (up to 3000; goalkeeper only +3500, the
/// others at least −2000); on the pitch, the minutes played (minute − `+0x1f`) below 60 × 40.
pub(crate) unsafe fn match_fatigue_offset(reg: MatchPlayer, goalkeeper: i32) -> i32 {
    let m = reg.engine();
    let human = if m.period_end() == period::FIRST_HALF_END {
        true
    } else {
        let side = reg.side();
        let base = Engine((m.addr() as isize + side as isize * 0xd0) as usize);
        let t = base.side0_manager();
        let n = read_i32(G_STAFF_COUNT);
        if t.is_null() || read_i32(t.addr()) < n - 0x10 || read_i32(t.addr()) >= n || t.classification() == 0 {
            false
        } else {
            let tecnicos = read_usize(G_HUMAN_MANAGER);
            if exe::human_manager_is_on_holiday(tecnicos, t) != 0 {
                false
            } else if (base.side0_manager_tactic_slot() as u8) != 0 {
                let c = read_usize(m.r#match() + if side == 0 { 0x1c } else { 0x20 });
                if exe::database_is_national_team(c) != 0 {
                    true
                } else {
                    exe::human_manager_controls_reserve_team(tecnicos, base.side0_manager()) != 0
                }
            } else {
                true
            }
        }
    };
    let extra = if human && *(reg.addr() as *const i8) < 0xb { -2000 } else { 0 };
    let state = reg.position();
    let minute = m.clock() as i32 / 11;
    if state == -1 {
        let e = extra + ((60 - minute) * 150).min(3000);
        if goalkeeper > 1 { e + 0xdac } else { e.max(-2000) }
    } else if state >= 0 {
        let jogados = (minute as u8).wrapping_sub(reg.minute_on() as u8) as i8;
        if jogados < 0x3c { extra + (jogados as i32 - 60) * 40 } else { extra }
    } else {
        extra
    }
}

/// Adjustment by tactic instructions (flag 0x8000): instruction bits, marking and tactic vs the player's attributes.
pub(crate) unsafe fn tactic_instruction_adjust(f: PlayerSheet, lo: u32, b1: u8, tactic: u32, marked: bool, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f.addr(), k);
    let x = |k: usize, c: usize| ftol((fb(k) - 10) as f64 * read_f64(c));
    let t13 = tactic as u8;
    let um = lo & 1 != 0;
    if lo & 4 != 0 || (um && t13 & 2 != 0) {
        l4c += fb(player_sheet::PASSING) / 7 - x(0x40, 0x96d5e8);
    } else if lo & 8 != 0 || (um && t13 & 4 != 0) {
        l4c += x(0x36, 0x968e00) - x(0x3c, 0x968fe8) - x(0x2e, 0x968fe8);
    } else if lo & 0x10 != 0 || (um && t13 & 8 != 0) {
        l4c += x(0x36, 0x96a670) - x(0x3c, 0x96d5e8) - x(0x2e, 0x96d5e8);
    }
    if marked {
        let c = fb(player_sheet::STAMINA) - 15;
        let d = fb(player_sheet::WORK_RATE) - 15;
        let x1 = ftol((c.abs() * c) as f64 * read_f64(rdata::F64_NEG_0_4));
        let x2 = ftol((d.abs() * d) as f64 * read_f64(rdata::F64_0_4));
        l4c += fb(player_sheet::ANTICIPATION) / 10 + (x2 - x1);
    } else if b1 & 1 != 0 {
        l4c += fb(player_sheet::MOVEMENT) / 12 + (fb(player_sheet::VISION) / 12 - x(0x28, 0x96d5e8));
    }
    if b1 & 2 != 0 {
        l4c += fb(player_sheet::POSITIONING) / 7;
    } else if b1 & 4 != 0 {
        l4c += fb(player_sheet::MARKING) / 7;
    }
    if lo & 0x40 != 0 {
        l4c += ftol((fb(player_sheet::RIGHT_FOOT) - 20) as f64 * read_f64(rdata::F64_0_2857142857));
    } else if lo & 0x100 != 0 {
        l4c += ftol((fb(player_sheet::LEFT_FOOT) - 20) as f64 * read_f64(rdata::F64_0_2857142857));
    }
    if lo & 0x200000 != 0 {
        l4c += fb(player_sheet::FREE_ROLE) + fb(player_sheet::VISION) / 10 + fb(player_sheet::MOVEMENT) / 9 - 20;
    }
    if lo & 0x20000 != 0 {
        l4c += (fb(player_sheet::JUMPING) - 10) / 4;
    } else if lo & 0x40000000 != 0 {
        l4c += (10 - fb(player_sheet::JUMPING)) / 4;
    }
    if lo & 0x80000000 != 0 {
        l4c += (10 - fb(player_sheet::JUMPING)) / 4;
    } else if lo & 0x80000 != 0 {
        l4c += (fb(player_sheet::JUMPING) - 10) / 4;
    }
    if lo & 0x800000 != 0 {
        l4c += (fb(player_sheet::STAMINA) - 10) / 2 + fb(player_sheet::VISION) / 15 + fb(player_sheet::FINISHING) / 15;
    }
    if lo & 0x2000000 != 0 {
        l4c += fb(player_sheet::DRIBBLING) / 7;
    }
    if lo & 0x8000000 != 0 {
        l4c += (fb(player_sheet::TECHNIQUE) - 10) / 3 + (fb(player_sheet::STRENGTH) - 10) / 2;
    }
    if lo & 0x20000000 != 0 {
        l4c += (fb(player_sheet::FLAIR) - 10) / 2 + fb(player_sheet::LONG_SHOTS) / 14 + fb(player_sheet::FINISHING) / 14;
    }
    l4c
}

/// Holiday bonus: a player in the 16-pair holiday list gets 40000 + 200 x rating if in the 11 and position accepted,
/// else 200 x rating; weighted by player value.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn holiday_selection_bonus(p: Staff, club: Club, holiday: usize, pos: u32, base: i32, without_value: u32, value: i32, mut l4c: i32) -> i32 {
    let id = read_i32(p.addr());
    let Some(k) = (0..16usize).find(|&k| read_i32(holiday + 8 * k) == id) else {
        return l4c;
    };
    let accepted = || {
        if exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), (2) as u32, club.manager().addr()) == 0 {
            return true;
        }
        let w = read_i32(holiday + 8 * k + 4);
        w == pos as i16 as i32 || w == -1
    };
    let mut bonus;
    if k < 0xb && accepted() {
        bonus = (base * 200).max(10000) + 40000;
        if l4c < 0 {
            l4c = 0;
        }
    } else {
        bonus = (base * 200).max(0);
        if l4c < 0 {
            l4c /= 2;
        }
    }
    if without_value == 0 {
        let v = value as f64;
        let y = v * read_f64(rdata::F64_1ENEG_08);
        let z = if value >= 0x251c {
            y * read_f64(rdata::F64_100000000)
        } else if value >= 0x222e {
            y * v * read_f64(rdata::F64_10000)
        } else {
            v * (y * v)
        };
        bonus = ftol((z * read_f64(rdata::F64_0_0002) - read_f64(rdata::F64_1)) * bonus as f64);
    }
    if bonus > 0 {
        l4c += bonus;
    }
    l4c
}

/// sets up one side's tactics (manager, players, club, opponent, match, home side) and selects: at the human's club the
/// stored lines (`select_tactic_slot`/`get_active_tactic`) and the automatic `select_automatically` mode 1 or the regulars (`starters`); otherwise the
/// automatic mode 2 and the count of the home players; the substitutes by the bench and the club's tactical line.
/// Returns how many it selected
/// Original: `select_side` (0x6db500, match_man.cpp, thiscall, ret 0x24).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn select_side(t: usize, m: usize, tecnico: usize, rows: usize, players: usize, side: u32, club: usize, other: usize, game: usize, slots: *mut i8) -> i8 {
    let m = Engine(m);
    let game = Fixture(game);
    let u = |q: usize, o: usize| read_usize(q + o);
    if m.is_null() {
        match_man_fatal_error(0x69);
    }
    if rows == 0 {
        match_man_fatal_error(0x6b);
    }
    if players == 0 {
        match_man_fatal_error(0x6d);
    }
    if club == 0 {
        match_man_fatal_error(0x6f);
    }
    if other == 0 {
        match_man_fatal_error(0x71);
    }
    if tecnico != 0 {
        write_usize(t + match_manager::MANAGER, tecnico);
        write_usize(t + match_manager::MANAGER_SHEET, u(tecnico, staff::NON_PLAYER));
    } else {
        write_usize(t + match_manager::MANAGER, 0);
        write_usize(t + match_manager::MANAGER_SHEET, 0);
    }
    write_usize(t + match_manager::PLAYERS, players);
    write_u8(t + match_manager::SIDE, side as u8);
    write_usize(t + match_manager::SQUAD_ROWS, rows);
    write_usize(t + match_manager::CLUB, club);
    write_usize(t + match_manager::ENGINE, m.addr());
    write_usize(t + match_manager::FIXTURE, game.addr());
    write_usize(t + match_manager::BEST_PLAYER, 0);
    write_usize(t + match_manager::KEY_PLAYER_3, 0);
    write_usize(t + match_manager::KEY_PLAYER_2, 0);
    write_usize(t + 6, 0);
    write_usize(t + match_manager::TACTICS, m.addr() + engine::TACTICS + (side as u8 as i8 as isize * (formation_fields::SIZE as isize)) as usize);
    write_usize(t + match_manager::OPPONENT, other);
    let home = (club == u(game.addr(), fixture::HOME_CLUB) && (game.home_side_mark() as i8) == 0) || (club == u(game.addr(), fixture::AWAY_CLUB) && game.away_side_mark() == 0);
    if home {
        write_u8(t + match_manager::TACTIC_SLOT, 1);
        write_u8(t + match_manager::SELECTION_MODE, 0);
    } else {
        write_u8(t + match_manager::TACTIC_SLOT, 0);
        write_u8(t + match_manager::SELECTION_MODE, 3);
    }
    let sheet = NonPlayer(read_usize(t + match_manager::MANAGER_SHEET));
    if !sheet.is_null() {
        write_u16(t, sheet.current_reputation() as u16);
        write_u16(t + match_manager::MANAGER_HOME_REPUTATION, sheet.home_reputation() as u16);
        write_u16(t + match_manager::MANAGER_WORLD_REPUTATION, sheet.world_reputation() as u16);
    }
    let manager = read_usize(G_HUMAN_MANAGER);
    let n: i8;
    if read_u8(m.addr()) == 0 && (exe::database_get_club_manager(club) as u32) != 0 && human_club(club, 0, 1, 0, 0) != 0 {
        if is_second_national_team(club) != 0 {
            write_usize(t + match_manager::MANAGER, u(main_national_team(u(club, club::NATION)), 0xcf));
        }
        exe::human_manager_select_tactic_slot(manager, read_i8(t + match_manager::TACTIC_SLOT) as i32, club, u(club, club::MANAGER));
        let r = exe::human_manager_get_active_tactic(manager, u(club, club::MANAGER));
        if r == 0 || read_usize(t + match_manager::TACTICS) == 0 {
            match_man_fatal_error(0xbc);
        }
        core::ptr::copy_nonoverlapping(r as *const u8, read_usize(t + match_manager::TACTICS) as *mut u8, 0x18e3);
        let without_manager = |t: usize| {
            let x = exe::human_manager_get_stand_in_manager(read_usize(t + match_manager::CLUB));
            write_usize(t + match_manager::MANAGER, x);
            write_usize(t + match_manager::MANAGER_SHEET, if x != 0 { read_usize(x + staff::NON_PLAYER) } else { 0 });
            exe::match_man_pick_team(t, other, 1)
        };
        n = if read_u8(t + match_manager::TACTIC_SLOT) == 0 {
            starters(t) as i8
        } else if is_national_team(club) == 0 {
            if manager_9a05(manager, u(club, club::MANAGER)) == 0 { without_manager(t) } else { starters(t) as i8 }
        } else if manager_9a06(manager, u(club, club::MANAGER), core::ptr::null_mut()) == 0 {
            without_manager(t)
        } else {
            starters(t) as i8
        };
        *slots = (((game.squad_rules() as u8) & 0xf) + 0xb) as i8;
    } else {
        let x = read_usize(t + match_manager::MANAGER);
        if x != 0 {
            let base = read_i32(G_STAFF_COUNT);
            let id = read_i32(x);
            if id >= base - 0x10 && id < base && read_i8(x + staff::CLASSIFICATION) != 0 {
                let y = exe::human_manager_get_stand_in_manager(read_usize(t + match_manager::CLUB));
                write_usize(t + match_manager::MANAGER, y);
                write_usize(t + match_manager::MANAGER_SHEET, if y != 0 { u(y, staff::NON_PLAYER) } else { 0 });
            }
        }
        n = exe::match_man_pick_team(t, other, 2);
        *slots = 0;
        let total = ((game.squad_rules() as u8) & 0xf) + 0xb;
        let mut k = 0u8;
        while (k as i8) < total as i8 {
            let p = u(players + k as usize * 0x1be, 0x69);
            if p != 0 && *(p as *const i32) < read_i32(G_STAFF_COUNT) {
                *slots = (*slots).wrapping_add(1);
            }
            k += 1;
        }
    }
    let bench = ((game.squad_rules() as u8) >> 4) & 0xf;
    let sobra = (n as u8).wrapping_sub(0xb) as i8;
    let r = if bench as i8 >= sobra { sobra } else { bench as i8 };
    write_u8(t + match_manager::SUBSTITUTIONS_LEFT, if r <= 0 { 0 } else if (bench as i8) < sobra { bench } else { sobra as u8 });
    read_tactics_row(read_usize(G_TACTICS), read_usize(t + match_manager::CLUB), (t + match_manager::TACTICS_ROW) as *mut u8, read_i8(t + match_manager::TACTIC_SLOT) as i32 as u32);
    n
}

/// A player's current or potential ability as an observer sees it (reputation, noise and the observer's knowledge).
///
/// The ability of player `p` (2 = current, otherwise potential) as observer `o` (0 = the manager of club `c`, `+0xcf`)
/// sees it: the reputation `rep` (negative: via `0x5407e0`) / 50 mixes with the true ability (player sheet `+0x61`:
/// `+5` current, `+7` potential) and a pseudo-random noise (player id + `[0xdd7120]` + observer id) that shrinks with
/// the observer's knowledge (`0x544410` of sheet `+0x69`, levels `+0x17`/`+0x18`, up to 20). With the observer's sheet,
/// high knowledge and a favourable seed, the exact current ability.
/// Original: `perceived_ability` (0x6e86d0, match_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn perceived_ability(o: usize, p: usize, c: usize, kind: i8, rep: i16) -> i16 {
    let p = Staff(p);
    let o = Staff(o);
    let c = Club(c);
    let mut semente = read_i32(p.addr()).wrapping_add(read_i32(G_GAME_RANDOM_SEED));
    let mut rep = if rep > -1 {
        rep / 50
    } else {
        exe::database_get_staff_reputation_for_viewer(p, if o.is_null() { c.manager().addr() } else { o.addr() }, 0) / 50
    };
    let sheet = p.player();
    let age = if kind == 2 {
        0
    } else {
        ((p.age() as u8)).wrapping_sub(if (sheet.goalkeeper() as u8) == 0x14 { 0x1a } else { 0x17 }) as i8
    };
    let mut conhece = 0usize;
    if !o.is_null() {
        conhece = o.non_player().addr();
        semente = semente.wrapping_add(read_i32(o.addr()));
        if conhece != 0 && kind == 2 && rep > 0x32 {
            let k = chance_to_know(conhece, read_i8(conhece + non_player::JUDGEMENT)) as i8 as i32;
            if rep as i32 + (k - 10) * 10 > semente % 200 {
                return sheet.current_ability();
            }
        }
    }
    let mut certeza: i8 = 1;
    if conhece != 0 {
        let f = ((semente % 101) as f64 * read_f64(rdata::F64_0_01)) as f32 as f64;
        let x = if semente % 10 == 0 {
            f * read_f32(rdata::F32_20) as f64
        } else {
            let level = read_i8(conhece + if kind == 2 { 0x17 } else { 0x18 });
            (chance_to_know(conhece, level) as i8 as f64) * f
        };
        certeza = (ftol(x + read_f64(rdata::F64_0_5)) as i8).min(0x14);
    }
    let c32 = certeza as i32;
    let noise = (semente % ((0x15 - c32) * 2) + c32 - 0x15) as i16 as i32;
    let truth = if kind == 2 {
        sheet.current_ability()
    } else {
        if age > 0 {
            rep = rep.wrapping_sub((age as i16).wrapping_mul(age as i16));
        }
        sheet.potential_ability()
    } as i32;
    let weight = (c32 / 2 + 10) as i8 as i32;
    (((truth + noise) * weight + (20 - weight) * rep as i32) / 20) as i16
}

