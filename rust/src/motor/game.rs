use super::*;

/// The nation's coefficient history: five past years (+0xb0..+0xd0) and the current year (+0xd8).
pub(crate) unsafe fn nation_coefficient_history(na: Nation) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = read_f64(na.addr() + nation::FIFACOEFFICIENT91 + k * 8);
    }
    h
}

/// Month of the game date (Date get_month on 0xae2c90).
pub(crate) unsafe fn current_month() -> i32 {
    (exe::date_get_month(G_CURRENT_DATE) as i8) as i32
}

/// December step: adds the reputation bonus if the mark is below 12, shifts the coefficient history one year and zeroes
/// the current year and the mark.
pub(crate) unsafe fn roll_coefficient_year(na: Nation, h: &mut [f64; 6], mark: &mut u8) {
    covered!("cpt_6 December");
    if (*mark as i8) < 0xc {
        covered!("cpt_6 December bonus");
        h[5] = na.reputation() as i32 as f64 * read_f64(rdata::F64_0_05) * (0xc - *mark as i8 as i32) as f64 * read_f64(rdata::F64_3) + h[5]; // cpt6: the December bonus;
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: the current year zeroed
    *mark = 0;
}

/// the first time, each nation history gets the coefficient at +0xa8; on the table day of the month, the weighted
/// history sum in an array that is sorted and freed; on the December day, the current year gets reputation × 0.05 × (12
/// − mark) × 3 (mark < 12), the history moves one year and the current year and the mark are cleared
/// Original: `game_update_nation_coefficients` (0x5d5f20, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_update_nation_coefficients() {
    if read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT) != 0 {
        if (read_usize(G_NATION_COUNT) as i32) > 0 {
            let mut p = read_usize(G_NATION_TABLE) + nation::FIFACOEFFICIENT95;
            let mut i = 0i16;
            loop {
                let v = read_f64(p - 0x28);
                for k in 0..5usize {
                    write_f64(p - k * 8, v);
                }
                i = i.wrapping_add(1);
                p += 0x122;
                if i as i32 >= read_usize(G_NATION_COUNT) as i32 {
                    break;
                }
            }
        }
        write_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT, 0);
    }
    let today = read_usize(G_CURRENT_DATE) as i16;
    if read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_TABLE + (current_month() * 2) as usize) != today {
        return;
    }
    let array = exe::crt_malloc((read_usize(G_NATION_COUNT).wrapping_mul(0xc)) as u32);
    if array == 0 {
        show_error_at(0x9cd2a4, 0x156d);
        return;
    }
    let dez = read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_DECEMBER) == read_i16(G_CURRENT_DATE); // cpt6: the December day
    let mut n = read_usize(G_NATION_COUNT) as i32;
    if n > 0 {
        let mut na = read_usize(G_NATION_TABLE);
        let mut i = 0i16;
        loop {
            let e = array + i as i32 as usize * 0xc;
            write_usize(e, na);
            let s = read_f64(na + nation::FIFACOEFFICIENT93) * read_f64(rdata::F64_0_6) + read_f64(na + nation::FIFACOEFFICIENT92) * read_f64(rdata::F64_0_4) + read_f64(na + nation::FIFACOEFFICIENT91) * read_f64(rdata::F64_0_2)
                + read_f64(na + nation::FIFACOEFFICIENT94) * read_f64(rdata::F64_0_8) + read_f64(na + nation::FIFACOEFFICIENT96) + read_f64(na + nation::FIFACOEFFICIENT95);
            write_f64(e + 4, s);
            if dez {
                let mut h = nation_coefficient_history(Nation(na));
                let mut mark = read_u8(na + nation::GAMES_PLAYED);
                roll_coefficient_year(Nation(na), &mut h, &mut mark);
                for (k, x) in h.iter().enumerate() {
                    write_f64(na + nation::FIFACOEFFICIENT91 + k * 8, *x);
                }
                write_u8(na + nation::GAMES_PLAYED, mark);
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = read_usize(G_NATION_COUNT) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    exe::crt_qsort(array, (n as usize) as u32, (0xc) as u32, 0x4afba0);
    exe::crt_free(array);
}

/// Returns 1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0.
///
/// `is_player_regen_day(day)`: 1 on days 0, 80, 173 and 250 of the year (the retirement search), otherwise 0. Called
/// from the game's day loop (0x5cda97, with the whole date: the day is the low word). The original returns only `al`
/// (`ah` keeps the day's high byte).
/// Original: `is_player_regen_day` (0x5d6250, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_player_regen_day(day: u32) -> u32 {
    let d = day as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}

/// Returns the sum of two signed bytes clamped between the given minimum and maximum.
///
/// `clamp_char_sum(a, b, min, max)`: the sum of the two signed bytes (16 bits), clamped between the other two. Returns
/// the original's `ax`: `min` or `max` extended, or the high byte of `max` extended with the sum in `al` (the 26
/// callers, in the players' daily pass, write only `al`).
/// Original: `clamp_char_sum` (0x5d61d0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn clamp_char_sum(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let s = (a as u8 as i8 as i16).wrapping_add(b as u8 as i8 as i16);
    let lo = min as u8 as i8 as i16;
    let hi = max as u8 as i8 as i16;
    if lo > s {
        lo as u16 as u32
    } else if hi < s {
        hi as u16 as u32
    } else {
        ((hi as u16 & 0xff00) | (s as u16 & 0xff)) as u32
    }
}

/// get_contract for a person, stored in G_CONTRACT_LOOKUP_RESULT.
pub(crate) unsafe fn game_get_contract_and_store(p: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// Club of a contract (by id at +4), or 0.
pub(crate) unsafe fn game_club_of_contract(c: usize) -> Club {
    if c == 0 {
        Club(0)
    } else {
        Club(club_record(read_usize(c + 4)))
    }
}

/// CONTRACT_MANAGER get_contract_for_club (`exe::contract_manager_get_contract_for_club`).
pub(crate) unsafe fn game_contract_for_club(p: usize, club: Club) -> Contract {
    Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club))
}

/// staff_is_disliked_by (`exe::staff_is_disliked_by`): does `pessoa` dislike `alvo`.
pub(crate) unsafe fn is_disliked_by(target: usize, person: Staff) -> u32 {
    exe::staff_is_disliked_by(target, person)
}

/// The shared end of the friction routines (`game_update_squad_relationships` and `game_check_club_dressing_room_unrest`): finds a squad mate the person dislikes and, on the
/// rare date condition, posts the complaint and sets the link mark.
///
/// The first squad mate (+0xd7, 50 slots) the person does not like; if found, and only when day % 14 + the person's id
/// gives zero (in practice the person with id 0, every 14 days), the complaint (`exe::news_post_dressing_room_unrest`) against the manager (+0xcf,
/// type 0), the assistant (+0xd3, 1) or someone else (2) and the mark on link `k` (`exe::staff_contract_apply_event_flags` with 0x2000, 0x1000 or
/// 0x800).
pub(crate) unsafe fn dressing_room_friction(p: Staff, club: Club, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = club.squad(j);
        if q != p.addr() && q != 0 && is_disliked_by(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = club.squad(j);
    if q == 0 {
        return;
    }
    if ((read_i16(G_CURRENT_DATE) as i32) % 0xe).wrapping_add(read_usize(p.addr()) as i32) != 0 {
        return;
    }
    covered!("cps_5 friction with the manager");
    let (tipo, marca) = if q == club.manager().addr() {
        (0, 0x2000)
    } else if q == club.assistant_manager() {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    exe::news_post_dressing_room_unrest(p, q, tipo);
    exe::staff_contract_apply_event_flags(k, marca, 0);
}

/// Updates the relationships in the squads: ten random pairs of team-mates: a disliked one stops being disliked and may
/// fight (`exe::news_post_feud_settled`); otherwise becomes disliked, liked or stops being liked; every 50 days, random
/// clubs go through the dressing room friction; on the other days each player, by attribute draws, may reach friction
/// with the manager.
///
/// `game_update_squad_relationships()`, every day from the game's day loop. (1) Ten times: a random club and two random
/// squad slots (`club::SQUAD`, 50); different people and random(the first one's `.professionalism()`) = 0: if the first
/// dislikes the second, he stops disliking him (`exe::staff_remove_disliked_staff`) and, at the same contract club and
/// with the link flagged in `.grievances()` & 0x800, the fight (`exe::news_post_feud_settled`); otherwise, 1/3 start
/// disliking (`exe::staff_add_disliked_staff`), 1/3 of the rest start liking (`exe::staff_add_favoured_staff`) and the
/// rest stop liking (`exe::staff_remove_favoured_staff`). (2) Day (`G_CURRENT_DATE`) a multiple of 50:
/// `G_CLUB_COUNT`/100 random clubs go through the dressing room friction (`exe::game_check_club_dressing_room_unrest`).
/// On the other days, each person (except the last 17) with a player record and preferences, a contract and the club's
/// link with the `club.squad(10)` person: three random draws (`.professionalism()`, `.temperament()`, `.pressure()`,
/// each + 5) filter; the sum of five attributes minus 3, 5, 15, 15 by draws that came out zero and 5 if the player's
/// reputation (`.current_reputation()`) exceeds 4500 and the club's (`.reputation()`) gives the draw that, when zero,
/// leads to friction with the manager.
/// Original: `game_update_squad_relationships` (0x5d6d10, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_update_squad_relationships() {
    for _ in 0..10 {
        let club = club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize);
        let a = Staff(read_usize(club.wrapping_add(friendly_random(0x32) as usize * 4) + club::SQUAD));
        let b = read_usize(club.wrapping_add(friendly_random(0x32) as usize * 4) + club::SQUAD);
        if a.is_null() || b == 0 || a.addr() == b {
            continue;
        }
        if friendly_random(a.professionalism() as i32) != 0 {
            continue;
        }
        if is_disliked_by(b, a) != 0 {
            exe::staff_remove_disliked_staff(a, b);
            let c = game_get_contract_and_store(a.addr());
            if c == 0 || game_club_of_contract(c) == Club(0) {
                continue;
            }
            let ca = game_club_of_contract(game_get_contract_and_store(a.addr()));
            let cb = game_club_of_contract(game_get_contract_and_store(b));
            if ca != cb {
                continue;
            }
            let k = game_contract_for_club(a.addr(), Club((game_club_of_contract(game_get_contract_and_store(a.addr()))).addr()));
            if k.is_null() || (k.grievances() as usize) & 0x800 == 0 {
                continue;
            }
            exe::news_post_feud_settled(a, b);
        } else if friendly_random(3) == 0 {
            exe::staff_add_disliked_staff(a, b, 1);
        } else if friendly_random(3) == 0 {
            exe::staff_add_favoured_staff(a, b, 1);
        } else {
            exe::staff_remove_favoured_staff(a, b);
        }
    }
    if (read_i16(G_CURRENT_DATE) as i32) % 0x32 == 0 {
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 / 100 {
            exe::game_check_club_dressing_room_unrest(club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = read_usize(G_STAFF_TABLE);
    let end = base.wrapping_add((read_usize(G_STAFF_COUNT).wrapping_sub(17)).wrapping_mul(staff::SIZE));
    let mut p = base;
    while p < end {
        player_vs_manager_check(Staff(p));
        p += 0x6e;
    }
}

/// One person of the non-50th-day loop: rolls the player's attitude toward the club's manager and ends with
/// dressing_room_friction [inference].
pub(crate) unsafe fn player_vs_manager_check(p: Staff) {
    if p.player().is_null() || p.staff_preferences() == 0 {
        return;
    }
    let c = game_get_contract_and_store(p.addr());
    if c == 0 {
        return;
    }
    let club = Club((game_club_of_contract(c)).addr());
    if club.is_null() {
        return;
    }
    // original defect, reproduced: the contract of squad slot 10, not the player's own (docs/DEFEITOS_DO_ORIGINAL.md)
    let k = game_contract_for_club(club.squad(10), club);
    if k.is_null() {
        return;
    }
    if friendly_random((p.professionalism() as i32) + 5) != 0 && friendly_random((p.temperament() as i32) + 5) != 0 && friendly_random((p.pressure() as i32) + 5) != 0 {
        return;
    }
    let mut e = (p.professionalism() as i32) + ((p.pressure() as i32) + (p.loyalty() as i32)) + ((p.sportsmanship() as i32) + ((p.age() as i8) as i32));
    covered!("cps_5 drawn manager");
    if friendly_random(p.loyalty() as i32) == 0 { // cps5: the first discount
        e -= 3;
    }
    if friendly_random(p.pressure() as i32) == 0 {
        e -= 5;
    }
    if (k.grievances() as usize) != 0 || friendly_random(0x64) == 0 {
        if friendly_random(p.professionalism() as i32) == 0 {
            e -= 0xf;
        }
        if friendly_random(p.temperament() as i32) == 0 {
            e -= 0xf;
        }
    }
    let rep = p.player().current_reputation();
    if rep > 0x1194 && rep > club.reputation() {
        e -= 5;
    }
    if friendly_random(e) >= 1 {
        return;
    }
    if p.staff_preferences() == 0 {
        return;
    }
    dressing_room_friction(p, club, k.addr());
}

// simplification: rolls and touches the preferences (0x547a10, 0x547af0, 0x547c10, 0x547c50), quarrel and complaint: original only;
// the copy takes the branches (runs every day).

/// each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the
/// complaint against the manager, the assistant or another (`exe::news_post_dressing_room_unrest`) and the flag on the contract (`exe::staff_contract_apply_event_flags`)
/// Original: `game_check_club_dressing_room_unrest` (0x5d7190, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_check_club_dressing_room_unrest(club: usize) {
    let club = Club(club);
    for i in 0..0x32usize {
        let p = Staff(club.squad(i));
        if p.is_null() {
            continue;
        }
        let k = game_contract_for_club(p.addr(), club);
        if k.is_null() {
            continue;
        }
        if p.staff_preferences() == 0 {
            continue;
        }
        dressing_room_friction(p, club, k.addr());
    }
}

// simplification: the complaint (0x587c60) and the mark on the link (0x867700) do not run twice: original only; the copy carries the
// friction branch.

/// True if the person has a valid id, belongs to the club (+0x39) and has a player record.
pub(crate) unsafe fn is_club_squad_player(p: Staff, club: usize) -> bool {
    !p.is_null() && (read_usize(p.addr()) as i32) < read_usize(G_STAFF_COUNT) as i32 && p.club_job() == Club(club) && !p.player().is_null()
}

/// Spreads morale through the squads by leadership: one club in seven each day (not national teams): the squad's
/// leadership and leadership × morale are summed; each player draws from them and his own morale, and his morale goes
/// up or down by one (`staff_adjust_morale`).
///
/// `game_spread_squad_morale_by_leadership()`, every day from the game's day loop. Each club that is not a national
/// team (`exe::database_is_national_team`), with random(7) = 0: sums the leadership (player `player_sheet::LEADERSHIP`)
/// and leadership × morale of the link (`.morale()`) over the squad (`club::SQUAD`, 50 slots); fewer than 16 players
/// count 10 for each one missing. With a non-zero sum, each player draws (2 × Σ leadership × morale) / (his leadership
/// + sum) plus random(morale / 12), as a signed byte: negative, random(s) < random(−5) lowers the morale
/// (`staff_adjust_morale` with −1); positive, random(s) > random(5) raises it (+1).
/// Original: `game_spread_squad_morale_by_leadership` (0x5ce0b0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_spread_squad_morale_by_leadership() {
    let morale: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let n = read_usize(G_CLUB_COUNT);
    let mut club = read_usize(G_CLUB_TABLE);
    let end = club.wrapping_add(n.wrapping_mul(club::SIZE));
    while club < end {
        let slots = club + club::SQUAD;
        if exe::database_is_national_team(club) == 0 && friendly_random(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = read_usize(slots + i * 4);
                if !is_club_squad_player(Staff(p), club) {
                    continue;
                }
                let c = game_contract_for_club(p, Club(club));
                if c.is_null() {
                    continue;
                }
                let l = read_i8_as_i32(read_usize(p + staff::PLAYER) + player_sheet::LEADERSHIP);
                soma = soma.wrapping_add((c.morale() as i32) * l);
                lideranca += l;
                q += 1;
            }
            if q < 0x10 {
                lideranca += (0x10 - q) * 10;
            }
            for i in 0..0x32usize {
                if lideranca == 0 {
                    break;
                }
                let p = read_usize(slots + i * 4);
                if !is_club_squad_player(Staff(p), club) {
                    continue;
                }
                let c = game_contract_for_club(p, Club(club));
                let m = if c.is_null() { 0 } else { c.morale() as i32  };
                let l = read_i8_as_i32(read_usize(p + staff::PLAYER) + player_sheet::LEADERSHIP);
                covered!("cps_5 drawn morale");
                let r1 = friendly_random(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = friendly_random(m / 12); // cps5: the morale part
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = friendly_random(s as i32);
                    if e < friendly_random(-5) {
                        morale(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = friendly_random(s as i32);
                    if e > friendly_random(5) {
                        covered!("cps_5 morale rises");
                        morale(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        club = club.wrapping_add(club::SIZE);
    }
}

// simplification: rolls random numbers and changes morale (0x544710, which also rolls): original only; the copy takes the branches.

/// Club record of a contract (G_CLUB_TABLE by id at +4), or 0.
pub(crate) unsafe fn contract_club_record(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    }
}

/// Days from the person's date at +0x3e to today.
pub(crate) unsafe fn days_since_staff_date(p: Staff) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32)
}

/// Checks players missing training: n/250 consecutive people from a random one; a present player with training and
/// without a serious injury misses training (`exe::awol_miss_training`) by a random draw on the sum of attributes, work
/// rate and liking of the manager (more likely when newly arrived) or by the contract flags.
///
/// `game_check_players_missing_training()`, every day from the game's day loop. Starting from a random person (random(n
/// − n/250 − 1)), n/250 consecutive people: a player (`.player()`, `.job_for_club()` = `job_kind::PLAYER`) with a
/// contract and a club, present (`exe::awol_is_player_awol`), with the training record
/// (`exe::training_manager_get_player_schedule`, `training_schedule::TYPE`) and without a serious injury
/// (`exe::injury_get_player_injury_status` ≠ 2). (1) With `.professionalism()` < 8, player `.leadership()` < 8 and the
/// sum (player `.leadership()`, `.age()`, `.professionalism()`, player `.work_rate()`, −5 if `.professionalism()` is 1,
/// −5 if `.work_rate()` is 1, −5 if he dislikes the manager (`club::MANAGER`), +2 if he likes him) < 43:
/// random(`.professionalism()` + 100, 50 or 25 depending on the days since `.date_joined_club()`: < 30, < 90, < 180)
/// coming out zero is a miss. (2) Otherwise, with `.professionalism()` < 10 and a link with the club: the link flags
/// (`contract::GRIEVANCES`: `grievances::UNHAPPY_WITH_TRAINING`, 0x30000; with `.professionalism()` < 5, 0x7800) give
/// random(`.professionalism()`) draws that, when zero, are a miss. The miss: `exe::awol_miss_training`.
/// Original: `game_check_players_missing_training` (0x5d6aa0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_check_players_missing_training() {
    let n = read_usize(G_STAFF_COUNT) as i32;
    let q = n / 250;
    covered!("cps_6 absences of the day");
    let r = friendly_random(n - q - 1); // cps6: the first person
    let mut current = read_usize(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(staff::SIZE));
    for _ in 0..q {
        let p = Staff(current);
        current = current.wrapping_add(0x6e);
        if p.player().is_null() || p.job_for_club() != job_kind::PLAYER {
            continue;
        }
        let c = game_get_contract_and_store(p.addr());
        if c == 0 {
            continue;
        }
        let club = contract_club_record(c);
        if club == 0 {
            continue;
        }
        if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
            continue;
        }
        let t = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
        if t == 0 || read_u8(t + training_schedule::TYPE) == 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        let jog = p.player();
        let cl = p.professionalism() as u8;
        let dl = jog.work_rate() as u8;
        let mut bl = (jog.leadership() as u8).wrapping_add(p.age() as u8).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if exe::staff_is_disliked_by(read_usize(club + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if exe::staff_is_favoured_by(read_usize(club + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut and_foul = false;
        let a = p.professionalism() as u8;
        if (a as i8) < 8 && ((p.player().leadership() as u8) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if days_since_staff_date(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if days_since_staff_date(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if days_since_staff_date(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            covered!("cps_6 absence by sum");
            if friendly_random(s as i8 as i32) == 0 { // cps6: the random draw of the sum
                and_foul = true;
            }
        }
        if !and_foul {
            if (p.professionalism() as i32) >= 0xa {
                continue;
            }
            let club = contract_club_record(game_get_contract_and_store(p.addr()));
            let v = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            if v == 0 {
                continue;
            }
            if read_usize(v + contract::GRIEVANCES) & (grievances::UNHAPPY_WITH_TRAINING as usize) != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else if read_usize(v + contract::GRIEVANCES) & 0x30000 != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else {
                let a = p.professionalism() as i32;
                if a >= 5 || read_usize(v + contract::GRIEVANCES) & 0x7800 == 0 || friendly_random(a) != 0 {
                    continue;
                }
                and_foul = true;
            }
        }
        if and_foul {
            covered!("cps_6 missed training");
            exe::awol_miss_training(read_usize(G_AWOL), p); // cps6: a falta
        }
    }
}

// simplification: draws and the foul (0x41a360) fines, draws and changes the form: original only; the copy carries the branches.

/// Gives the day to each competition in the list (virtual `comp_vtable::UPDATE_FOR_DATE` with the game date and the
/// mark); with the mark, the next step of `G_FIXTURE_MANAGER` (`exe::fixture_manager_daily_update`).
///
/// `game_update_comps_for_date(mark)`: each competition in the list (`G_COMP_OBJECT_TABLE`, count `G_COMP_OBJECT_COUNT`
/// re-read on each iteration) receives the day (virtual `comp_vtable::UPDATE_FOR_DATE` with the game date
/// `G_CURRENT_DATE` and the mark); with the mark, the next step of `G_FIXTURE_MANAGER`
/// (`exe::fixture_manager_daily_update`, which the original calls as a tail jump). Called from the game's day loop
/// (with 1).
/// Original: `game_update_comps_for_date` (0x5ce060, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_update_comps_for_date(mark: u32) {
    let m = mark as u8 as i8;
    let mut i = 0i32;
    while i < read_usize(G_COMP_OBJECT_COUNT) as i32 {
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE) + i as usize * 4);
        if c != 0 {
            let day: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::UPDATE_FOR_DATE));
            day(c, G_CURRENT_DATE, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        covered!("cps_6 competitions of the day");
        exe::fixture_manager_daily_update(read_usize(G_FIXTURE_MANAGER));
    }
}

// simplification: advances the day of all competitions (virtual) and of [0xae2a58]: original only; the copy takes the branch.

/// Address of the club's board confidence word (+6 of its 73-byte row in [this]).
pub(crate) unsafe fn board_confidence_ptr(this: usize, club: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(club).wrapping_mul(0x49)) + 6
}

/// Adds the delta to the board's confidence in the club (+6 of the 73-byte club row), held to 1..10000.
///
/// `manager_manager_adjust_board_confidence(club, delta)`: adds `delta` (word) to the board's confidence in the club
/// (+6 of the MANAGER_MANAGER row `[0xb67a44]`), clamped to 1..10000. Null club: nothing. Called from the game's day
/// loop (with −5).
/// Original: `manager_manager_adjust_board_confidence` (0x5cdde0, game.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn manager_manager_adjust_board_confidence(this: usize, club: usize, delta: u32) {
    if club == 0 {
        return;
    }
    let c = board_confidence_ptr(this, club);
    write_i16(c, read_i16(c).wrapping_add(delta as u16 as i16));
    let v = read_i16(c);
    if v < 1 {
        write_i16(c, 1);
    } else if v > 0x2710 {
        write_i16(c, 0x2710);
    }
}

/// the shortlist manager kept in the transfer manager
/// Original: `transfer_manager_get_shortlist_manager` (0x5d5ab0, game.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_shortlist_manager(manager: usize) -> usize {
    let manager = TransferManagerFields(manager);
    manager.shortlist_manager()
}

/// the player's 6-byte entry in the training table
/// Original: `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn training_manager_get_player_entry(training: usize, player: usize) -> usize {
    let player = Staff(player);
    let table = read_usize(training + training_manager_fields::PLAYER_ENTRIES);
    let id = read_i32(player.player().addr());
    (table as isize + id as isize * 6) as usize
}

/// Pointer to the training load byte (+0x1e) of a player's 31-byte condition record.
pub(crate) unsafe fn training_load_byte(injuries: usize, id: *const i32) -> *mut u8 {
    (read_usize(injuries) as isize + *id as isize * 31 + (physical_record::PRONENESS as isize)) as *mut u8
}

/// the f64 average `player_stats_fields::FLAGGED_RATING_SUM`/`player_stats_fields::FLAGGED_RATED_COUNT` or
/// `player_stats_fields::RATING_SUM`/`player_stats_fields::RATED_COUNT` (zero divisor: 6.8, `rdata::F64_6_8`)
/// Original: `game_average` (0x5d5a60, game.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn game_average(t: usize, second: u8) -> f64 {
    let (n, q) = if second != 0 { (read_f64(t + player_stats_fields::FLAGGED_RATING_SUM), read_f64(t + player_stats_fields::FLAGGED_RATED_COUNT)) } else { (read_f64(t + player_stats_fields::RATING_SUM), read_f64(t + player_stats_fields::RATED_COUNT)) };
    if q == read_f64(rdata::F64_0) || q.is_nan() {
        return read_f64(rdata::F64_6_8);
    }
    n / q
}

compare_value!(compare_contract_value, contract_value, IDX_CONTRACT_VALUE, (p: *const i32) -> u32);

/// the game CRT pow (0x945ff0) of two doubles, result in st0
/// Original: `pow_double` (0x5d5a20, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn pow_double(base: f64, expoente: f64) -> f64 {
    game_pow(base, expoente)
}

/// player data qsort: current ability (+5), then potential (+7), ascending
/// Original: `compare_players_by_ability` (0x5d6220, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_ability(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    if (read_i16(pa + 5) as i32) == (read_i16(pb + 5) as i32) {
        return (read_i16(pa + 7) as i32) - (read_i16(pb + 7) as i32);
    }
    (read_i16(pa + 5) as i32) - (read_i16(pb + 5) as i32)
}

