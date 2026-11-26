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
                        exe::staff_adjust_morale(p, (-1) as i32 as u32, 1);
                    }
                } else if s > 0 {
                    let e = friendly_random(s as i32);
                    if e > friendly_random(5) {
                        covered!("cps_5 morale rises");
                        exe::staff_adjust_morale(p, (1) as i32 as u32, 1); // cps5: sobe o moral
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
            let v = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
            if v.is_null() {
                continue;
            }
            if (v.grievances() as usize) & (grievances::UNHAPPY_WITH_TRAINING as usize) != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else if (v.grievances() as usize) & 0x30000 != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else {
                let a = p.professionalism() as i32;
                if a >= 5 || (v.grievances() as usize) & 0x7800 == 0 || friendly_random(a) != 0 {
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

/// Runs each of the 35 season entries (`G_LEAGUE_TABLE`, 72 bytes each) on its update day (`SEASON_END_DAY`, +0x15)
/// and archives its records the day before.
///
/// On first use it builds the static `LAST_DATE` (`Date::Date`, at-exit destructor). The weight of the pass is 2 for
/// each entry whose update day is today and 4 for each whose update day is tomorrow (`Date::add_days(today, 1)`); with
/// a weight of 10 or more and the front end interactive, "Updating game data" goes to the screen manager as progress
/// (step 0 first, then one step per entry run and per archive call).
///
/// An entry whose update day is today: unless it has a nation whose league is not selected (`league_selected`, +0x11c,
/// 0), and unless its start year (+0x35) is already today's year, the start year moves on by the entry's year step
/// (+0x1d, signed byte), the end year (+0x44) too, and the start date (+0x2a) and end date (+0x39) are rebuilt from day
/// (+0x32 / +0x41), month (+0x34 / +0x43), year and weekday (+0x37 / +0x46). For entry 3, every person's
/// `squad_selected_for` is cleared. Then each of its competitions (count +0xc, array +0x10) that is complete in every
/// stage (vtable 0xc) and does not skip the reset (vtable 0x2c, which may mark the competition) restarts: for a league
/// division (`comp_util_is_league_division` of the record's id), the average attendance of each stage linked by +0x20
/// (−1 or an empty slot of `G_COMP_OBJECT_TABLE` ends); then `START_NEW_SEASON`, the discipline's new season for the
/// competition record, and, the first time on a given date (`LAST_DATE` differs from today), the SimulatedStats object
/// (`G_SIMULATED_STATS`) is freed and built again and `LAST_DATE` becomes today.
///
/// An entry whose update day is tomorrow: `Club_Records`, `PlayerStats`, `FinanceTable` and `Staff_Records` close the
/// nation's season (each `end_season_for_nation(entry)`), with a progress step after each.
/// Original: `game_season_update_pass` (0x5d5ae0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_season_update_pass() {
    let flag = read_u8(season_pass::INIT_FLAG);
    if flag & 1 == 0 {
        write_u8(season_pass::INIT_FLAG, flag | 1);
        exe::date_date_54a110(season_pass::LAST_DATE);
        exe::crt_atexit(season_pass::FN_LAST_DATE_DTOR);
    }
    let mut tomorrow = [0u16; 4];
    let mut weight = 0i32;
    for k in 0..35usize {
        let day = read_u16(G_LEAGUE_TABLE + k * 72 + key_nation_record::SEASON_END_DAY) as i32;
        if read_i16(G_CURRENT_DATE) as i32 == day {
            weight += 2;
        } else if read_i16(exe::date_add_days(G_CURRENT_DATE, tomorrow.as_mut_ptr() as usize, 1)) as i32 == day {
            weight += 4;
        }
    }
    let mut text = [0u8; 0xb8];
    let mut step = 0i32;
    if weight >= 10 && read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
        let sprintf: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        sprintf(text.as_mut_ptr(), season_pass::TEXT_UPDATING_GAME_DATA);
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, step, weight as u32, -1);
    }
    for k in 0..35usize {
        let e = G_LEAGUE_TABLE + k * 72;
        let day = read_u16(e + key_nation_record::SEASON_END_DAY) as i32;
        if read_i16(G_CURRENT_DATE) as i32 != day {
            if read_i16(exe::date_add_days(G_CURRENT_DATE, tomorrow.as_mut_ptr() as usize, 1)) as i32 == day {
                season_pass_day_before(e, &text, &mut step, weight);
            }
            continue;
        }
        covered!("nma_2 season start");
        let nation = read_usize(e);
        if (nation == 0 || Nation(nation).league_selected() != 0) && read_u16(e + season_entry::START_YEAR) != read_u16(G_CURRENT_DATE_YEAR) {
            season_pass_move_dates(e, nation);
        }
        if k == 3 {
            let mut i = 0i32;
            while i < read_i32(G_STAFF_COUNT) {
                staff_at(i as isize).set_squad_selected_for(0); // nma2: entry 3 clears the selections
                i += 1;
            }
        }
        step += 1;
        season_pass_progress(&text, step, weight);
        let mut j = 0i32;
        while j < read_i32(e + season_entry::COMP_COUNT) {
            let comp = read_usize(read_usize(e + season_entry::COMPS) + j as usize * 4);
            if comp != 0 {
                season_pass_restart_comp(comp);
            }
            j += 1;
        }
        step += 1;
        season_pass_progress(&text, step, weight);
    }
}

/// Comparer for `game_season_update_pass`.
pub(crate) unsafe extern "C" fn compare_game_season_update_pass() {
    // simplification: starts the seasons, rebuilds the SimulatedStats and archives the records (it cannot run twice):
    // original only; the copy is proven in Rust mode (careers and mutations)
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_SEASON_UPDATE_PASS)()
}

/// Sends the "Updating game data" progress step (`ScreenManager::send_network_status_message(text, step, total, -1)`),
/// only when the pass is heavy (weight 10 or more) and the front end is interactive.
pub(crate) unsafe fn season_pass_progress(text: &[u8; 0xb8], step: i32, weight: i32) {
    if weight >= 10 && read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, step, weight as u32, -1);
    }
}

/// The day before an entry's update day: the nation's records are archived (club records, player stats, finances,
/// staff records), one progress step after each.
pub(crate) unsafe fn season_pass_day_before(e: usize, text: &[u8; 0xb8], step: &mut i32, weight: i32) {
    covered!("nma_2 day before");
    let by_entry = |f: usize, this: usize| {
        let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
        g(this, e);
    };
    by_entry(season_pass::FN_CLUB_RECORDS_END_SEASON, read_usize(G_CLUB_RECORDS)); // nma2: club records
    *step += 1;
    season_pass_progress(text, *step, weight);
    by_entry(season_pass::FN_PLAYER_STATS_END_SEASON, read_usize(G_PLAYER_STATS)); // nma2: player stats
    *step += 1;
    season_pass_progress(text, *step, weight);
    by_entry(season_pass::FN_FINANCE_END_SEASON, read_usize(G_FINANCE_TABLE)); // nma2: finances
    *step += 1;
    season_pass_progress(text, *step, weight);
    by_entry(season_pass::FN_STAFF_RECORDS_END_SEASON, read_usize(G_STAFF_RECORDS)); // nma2: staff records
    *step += 1;
    season_pass_progress(text, *step, weight);
}

/// Moves an entry's start and end dates on by the entry's year step (+0x1d), rebuilding both with `Date::Date(day,
/// month, year, weekday)`. The registers the original passes carry high bits the callee does not read (it reads the
/// day, the year and the weekday as words and the month as a byte); they are reproduced as far as they do not depend on
/// the stack.
pub(crate) unsafe fn season_pass_move_dates(e: usize, nation: usize) {
    covered!("nma_2 new dates");
    let span = read_i8(e + key_nation_record::SEASON_SPANS_YEAR) as i16;
    write_u16(e + season_entry::END_YEAR, read_u16(e + season_entry::END_YEAR).wrapping_add(span as u16));
    let eax = (nation as u32 & 0xffff_0000) | span as u16 as u32;
    let today_hi = read_i16(G_CURRENT_DATE) as i32 as u32 & 0xffff_0000;
    let year = (today_hi | read_u16(e + season_entry::START_YEAR) as u32).wrapping_add(eax); // nma2: the new start year
    write_u16(e + season_entry::START_YEAR, year as u16);
    let mut d = [0u16; 4];
    let p = exe::date_date_549f70(&mut d, (year & 0xffff_0000) | read_u16(e + season_entry::START_DAY) as u32, (eax & 0xffff_ff00) | read_u8(e + key_nation_record::START_MONTH) as u32, year, read_u16(e + season_entry::START_WEEKDAY) as u32);
    let mut c = [0u16; 4];
    let r = exe::date_operator_3d(e + key_nation_record::START_DATE, c.as_mut_ptr() as usize, p) as u32;
    let edx = read_u32(e + key_nation_record::START_DATE);
    let mut d2 = [0u16; 4];
    let p2 = exe::date_date_549f70(&mut d2, (r & 0xffff_0000) | read_u16(e + season_entry::END_DAY) as u32, (edx & 0xffff_ff00) | read_u8(e + season_entry::END_MONTH) as u32, read_u16(e + season_entry::END_YEAR) as u32, (r & 0xffff_0000) | read_u16(e + season_entry::END_WEEKDAY) as u32);
    let mut c2 = [0u16; 4];
    exe::date_operator_3d(e + key_nation_record::END_DATE, c2.as_mut_ptr() as usize, p2);
}

/// Restarts one of the entry's competitions on its update day, when all its stages are complete and it does not skip
/// the reset: the average attendance of every stage of a league division, the competition's `START_NEW_SEASON`, the
/// discipline's new season for it and, once a day, a new SimulatedStats.
pub(crate) unsafe fn season_pass_restart_comp(comp: usize) {
    let complete: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(comp) + season_pass::VT_ALL_STAGES_COMPLETE));
    if complete(comp) == 0 {
        return;
    }
    let skip: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + season_pass::VT_SKIP_SEASON_RESET));
    if skip(comp) != 0 {
        return;
    }
    covered!("nma_2 comp restart");
    if exe::comp_util_is_league_division(read_i32(read_usize(comp + 4))) != 0 {
        let attendance: unsafe extern "C" fn(usize) = core::mem::transmute(season_pass::FN_UPDATE_AVERAGE_ATTENDANCE);
        let mut stage = comp;
        loop {
            attendance(stage);
            let next = read_i32(stage + 0x20);
            if next == -1 {
                break;
            }
            stage = read_usize(read_usize(G_COMP_OBJECT_TABLE) + next as usize * 4);
            if stage == 0 {
                break;
            }
        }
    }
    let start: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::START_NEW_SEASON));
    start(comp); // nma2: the competition's new season
    let discipline: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(season_pass::FN_DISCIPLINE_NEW_SEASON);
    discipline(read_usize(G_DISCIPLINE), read_usize(comp + 4));
    if read_u16(G_CURRENT_DATE_YEAR) != read_u16(season_pass::LAST_DATE_YEAR) || read_u16(G_CURRENT_DATE) != read_u16(season_pass::LAST_DATE) {
        covered!("nma_2 simulated stats");
        let old = read_usize(G_SIMULATED_STATS);
        if old != 0 {
            let dtor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(season_pass::FN_SIMULATED_STATS_DTOR);
            dtor(old);
            exe::crt_operator_delete(old);
        }
        let n = exe::crt_operator_new(8);
        let fresh = if n != 0 {
            let ctor: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(season_pass::FN_SIMULATED_STATS_CTOR);
            ctor(n, 0)
        } else {
            0
        };
        write_usize(G_SIMULATED_STATS, fresh);
        let mut c = [0u16; 4];
        exe::date_operator_3d(season_pass::LAST_DATE, c.as_mut_ptr() as usize, G_CURRENT_DATE);
    }
}

/// Runs the game day by day from the game date to the chosen start date before the career begins.
///
/// First the player setup files (`player_setup_cfg_load` with the masks 0x100, 0x1000, 0x800 and 0x200, no error
/// reports). The progress total is the number of days to go: (start year − game year) × 365 − game day + start day.
/// Then, while the game date is before `G_GAME_START_DATE` (year unsigned, then day signed), each of the three slices of
/// a day (`G_CURRENT_DAY_SLICE` 0..2):
///
/// - a new year sets the fixtures' year (`exe::fixture_manager_set_year`);
/// - the matches, fixtures, staff records, competitions and contracts of the slice (`start_date_slice_matches`);
/// - on the third slice, the ages, the weather, the "Initialising Leagues" progress step and the world's daily updates
///   (`start_date_slice_world`: discipline, injuries, UEFA seeding, awards, competitions, host countries, the season
///   pass, the nation coefficients, national teams, finances, officials, managers, stadiums);
/// - the club records' news (`Club_Records::send_record_news`);
/// - the slice moves on; after the third, the date moves one day (`start_date_next_day`, with the FIFA rules check);
/// - the Windows messages (`exe::pump_messages`).
///
/// At the end `G_LAST_PROCESSED_DATE` is the day before the game date, the game phase byte 0xae2cfc becomes 2, the
/// computer clubs assess their squads (shortlists, `Shortlist_Manager` at `[G_TRANSFER_MANAGER] + 0x856`), the player
/// setup file is read with mask 0x203b and the squads' fitness is set (`Injury::init_squad_fitness`).
///
/// The original passes the year and the slice in registers whose high bits are leftovers; the callees read only the
/// word or the byte (`set_year` compares `bx`, `set_up_the_day` and the contracts read the byte).
/// Original: `game_run_to_start_date` (0x5cc850, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_run_to_start_date() {
    for mask in [0x100u32, 0x1000, 0x800, 0x200] {
        let load: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
        load(mask, 0);
    }
    let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k));
    let total = (start[1] as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32) * 365 - read_i16(G_CURRENT_DATE) as i32 + start[0] as i16 as i32;
    let (mut last_year, mut counter) = (0u16, 0i32);
    loop {
        let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k)); // re-read every day, as the original
        if !before_start_date(&start) {
            break;
        }
        let year = read_u16(G_CURRENT_DATE_YEAR);
        if year != last_year {
            exe::fixture_manager_set_year(read_usize(G_FIXTURE_MANAGER), year as u32);
            last_year = read_u16(G_CURRENT_DATE_YEAR);
        }
        let slice = read_u8(G_CURRENT_DAY_SLICE);
        start_date_slice_matches(slice);
        if read_u8(G_CURRENT_DAY_SLICE) == 2 {
            start_date_slice_world(&mut counter, total);
        }
        start_date_call0(start_date_fns::CLUB_RECORDS_SEND_RECORD_NEWS, read_usize(G_CLUB_RECORDS));
        let next = read_u8(G_CURRENT_DAY_SLICE).wrapping_add(1);
        write_u8(G_CURRENT_DAY_SLICE, next);
        if next > 2 {
            write_u8(G_CURRENT_DAY_SLICE, 0);
            start_date_next_day();
        }
        exe::pump_messages();
    }
    let (mut out, mut tmp) = ([0u16; 4], [0u16; 4]);
    let yesterday = exe::date_sub_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    exe::date_operator_3d(G_LAST_PROCESSED_DATE, tmp.as_mut_ptr() as usize, yesterday);
    write_u8(start_date_fns::G_GAME_PHASE, 2);
    start_date_call0(start_date_fns::ASSESS_CLUB_SQUADS, read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)); // the Shortlist_Manager
    let load: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    load(0x203b, 0);
    start_date_call0(start_date_fns::INJURY_INIT_SQUAD_FITNESS, read_usize(G_INJURY));
}

/// Comparer for `game_run_to_start_date`: it plays every day up to the start date and cannot run twice, so only the
/// original runs and the plan is checked afterwards: the game date reached the start date (when it was before it), the
/// last processed date is the day before, no match day is left, the slice is back to 0 and the phase byte is 2.
pub(crate) unsafe extern "C" fn compare_game_run_to_start_date() {
    let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k));
    let ran = before_start_date(&start);
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_RUN_TO_START_DATE)();
    let mut out = [0u16; 4];
    let yesterday = exe::date_sub_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    let last_ok = read_u16(G_LAST_PROCESSED_DATE) == read_u16(yesterday) && read_u16(G_LAST_PROCESSED_DATE + 2) == read_u16(yesterday + 2);
    let date_ok = !ran || (read_u16(G_CURRENT_DATE) == start[0] && read_u16(G_CURRENT_DATE_YEAR) == start[1] && read_u8(G_CURRENT_DAY_SLICE) == 0);
    let ok = last_ok && date_ok && read_usize(G_MATCH_DAY) == 0 && read_u8(start_date_fns::G_GAME_PHASE) == 2;
    SCOREBOARD.record(ok, || format!("game_run_to_start_date: ran {} date {}/{} slice {} (start {}/{}), last processed {}/{}, match day {:x}, phase {}",
        ran, read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR), read_u8(G_CURRENT_DAY_SLICE), start[0] as i16, start[1],
        read_i16(G_LAST_PROCESSED_DATE), read_u16(G_LAST_PROCESSED_DATE + 2), read_usize(G_MATCH_DAY), read_u8(start_date_fns::G_GAME_PHASE)));
}

/// `compare_shorts_ascending(a, b)`: qsort comparison of two 16-bit values, ascending: `*a − *b`, both sign-extended.
/// Used by the daily person pass (`game_daily_staff_update` 0x5ce2d0) to sort the two tables of the day's abilities
/// (current and potential) that the players' development reads.
/// Original: `compare_shorts_ascending` (0x5d6200, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    read_i16(a as usize) as i32 - read_i16(b as usize) as i32
}

/// Comparer for `compare_shorts_ascending`: the order of the original and of the copy.
pub(crate) unsafe extern "C" fn compare_compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i16, *const i16) -> i32>(IDX_COMPARE_SHORTS_ASCENDING)(a, b);
    let n = compare_shorts_ascending(a, b);
    SCOREBOARD.record(r == n, || format!("compare_shorts_ascending({}, {}) original={} rust={}", read_i16(a as usize), read_i16(b as usize), r, n));
    r
}

/// `game_date_static_init()`: static initialiser of the exe (runs in the CRT start-up, before WinMain, with the detours
/// already installed by the DLL load): builds the game date `G_CURRENT_DATE` with the default `Date` and registers its
/// destructor (`game_date_static_dtor` 0x5cc670, an empty body) with `exe::crt_atexit`. The game date is set for real
/// when a game is created or loaded.
/// Original: `game_date_static_init` (0x5cc650, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_date_static_init() {
    construct_static_date(G_CURRENT_DATE);
    exe::crt_atexit(0x5cc670usize); // game_date_static_dtor: an empty body
}

/// `last_processed_date_static_init()`: static initialiser of the exe, like `game_date_static_init`, for the last
/// processed date `G_LAST_PROCESSED_DATE` (destructor `last_processed_date_static_dtor` 0x5cc6a0, an empty body).
/// Original: `last_processed_date_static_init` (0x5cc680, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn last_processed_date_static_init() {
    construct_static_date(G_LAST_PROCESSED_DATE);
    exe::crt_atexit(0x5cc6a0usize); // last_processed_date_static_dtor: an empty body
}

/// Comparer for `game_date_static_init`.
pub(crate) unsafe extern "C" fn compare_game_date_static_init() {
    compare_static_date("game_date_static_init", original_fn::<unsafe extern "C" fn()>(IDX_GAME_DATE_STATIC_INIT), G_CURRENT_DATE);
}

/// Comparer for `last_processed_date_static_init`.
pub(crate) unsafe extern "C" fn compare_last_processed_date_static_init() {
    compare_static_date("last_processed_date_static_init", original_fn::<unsafe extern "C" fn()>(IDX_LAST_PROCESSED_DATE_STATIC_INIT), G_LAST_PROCESSED_DATE);
}

/// `this` routine of the exe called with no argument (thiscall).
pub(crate) unsafe fn start_date_call0(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(this)
}

/// Whether the game date is still before the start date `start` (day i16, year u16): the year unsigned first, then the
/// day of the year signed.
pub(crate) unsafe fn before_start_date(start: &[u16; 4]) -> bool {
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if start[1] != year {
        return start[1] > year;
    }
    start[0] as i16 > read_i16(G_CURRENT_DATE)
}

/// The first part of a day slice: the match day of the slice (built, played, ended and freed; `G_MATCH_DAY` points at
/// it meanwhile), the fixtures, the staff records, the competitions (`game_update_comps_for_date(0)`) and the
/// contracts.
pub(crate) unsafe fn start_date_slice_matches(slice: u8) {
    let day = exe::crt_operator_new(0x38);
    let day = if day != 0 {
        let set_up: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(start_date_fns::SET_UP_THE_DAY);
        set_up(day, G_CURRENT_DATE, slice as u32)
    } else {
        0
    };
    write_usize(G_MATCH_DAY, day);
    start_date_call0(start_date_fns::PLAY_THE_DAY, day); // the original calls it even on a null day
    exe::fixture_manager_daily_update(read_usize(G_FIXTURE_MANAGER));
    start_date_call_today(start_date_fns::STAFF_RECORDS_DAILY_UPDATE, read_usize(G_STAFF_RECORDS));
    game_update_comps_for_date(0);
    let day = read_usize(G_MATCH_DAY);
    if day != 0 {
        start_date_call0(start_date_fns::END_DAY, day);
        exe::crt_operator_delete(day);
    }
    write_usize(G_MATCH_DAY, 0);
    let contracts: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(start_date_fns::CONTRACT_MANAGER_DAILY_UPDATE);
    contracts(read_usize(G_CONTRACT_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, 1); // the slice read again
}

/// The third slice of a day: every person's age (`staff::AGE`, from `exe::database_get_staff_age` on the game date),
/// the weather, the "Initialising Leagues" progress step, then the world's daily updates in the original's order.
pub(crate) unsafe fn start_date_slice_world(counter: &mut i32, total: i32) {
    for i in 0..read_usize(G_STAFF_COUNT) as i32 {
        let p = staff_at(i as isize);
        p.set_age(exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16) as i16);
    }
    start_date_call0(start_date_fns::UPDATE_CITY_WEATHER, read_usize(G_WEATHER));
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(start_date_fns::G_STATUS_TEXT, start_date_fns::TEXT_INITIALISING_LEAGUES);
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, start_date_fns::G_STATUS_TEXT, *counter, total as u32, -1);
    *counter += 1;
    start_date_call_today(start_date_fns::DISCIPLINE_DAILY_UPDATE, read_usize(G_DISCIPLINE));
    start_date_call0(start_date_fns::INJURY_DAILY_UPDATE, read_usize(G_INJURY));
    exe::uefaseeding_update_season(read_usize(G_UEFA_SEEDING), 1);
    start_date_call_today(start_date_fns::AWARD_MANAGER_PROCESS_DATE, read_usize(G_AWARD_MANAGER));
    game_update_comps_for_date(1); // the competitions' day (phase 1) and the fixtures again
    start_date_call_today(start_date_fns::HOST_COUNTRY_DAILY_UPDATE, read_usize(G_HOST_COUNTRY));
    let cdecl0 = |f: usize| {
        let g: unsafe extern "C" fn() = core::mem::transmute(f);
        g()
    };
    cdecl0(start_date_fns::GAME_SEASON_UPDATE_PASS);
    cdecl0(start_date_fns::GAME_UPDATE_NATION_COEFFICIENTS);
    start_date_call_today(start_date_fns::NATIONAL_TEAMS_DAILY_UPDATE, read_usize(G_NATIONAL_TEAMS));
    start_date_call_today(start_date_fns::FINANCE_TABLE_DAILY_UPDATE, read_usize(G_FINANCE_TABLE));
    start_date_call0(start_date_fns::OFFICIALS_CLEAR_BUSY_FLAGS, read_usize(G_OFFICIALS_MANAGER));
    start_date_call0(start_date_fns::MANAGER_MANAGER_DAILY_UPDATE, read_usize(G_MANAGER_MANAGER));
    start_date_call_today(start_date_fns::STADIUM_DAILY_UPDATE, read_usize(G_STADIUM));
}

/// After the third slice the date moves on one day (`exe::date_add_days`, `exe::date_operator_3d`), and the FIFA
/// transfer rules (`G_FIFA_TRANSFER_RULES_FEATURE_ENABLED` on and not yet `G_FIFA_TRANSFER_RULES_IN_FORCE`) check their
/// date `G_TRANSFER_RULE_CHANGE_DATE`: already past (year unsigned, then day signed), the rules come into force
/// (`G_MAX_CONTRACT_LENGTH_YEARS` = 5, news `exe::news_post_transfer_system_change(1)`); exactly 25 days before it, the
/// warning news (`news_post_transfer_system_change(0)`).
pub(crate) unsafe fn start_date_next_day() {
    let (mut out, mut tmp) = ([0u16; 4], [0u16; 4]);
    let next = exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    exe::date_operator_3d(G_CURRENT_DATE, tmp.as_mut_ptr() as usize, next);
    if read_u8(G_FIFA_TRANSFER_RULES_FEATURE_ENABLED) == 0 || read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 0 {
        return;
    }
    let (rule_day, rule_year) = (read_i16(G_TRANSFER_RULE_CHANGE_DATE), read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2));
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if rule_year < year || (rule_year == year && rule_day < read_i16(G_CURRENT_DATE)) {
        covered!("sx11_1 transfer rules in force");
        write_u8(G_FIFA_TRANSFER_RULES_IN_FORCE, 1);
        write_u8(G_MAX_CONTRACT_LENGTH_YEARS, 5);
        exe::news_post_transfer_system_change(1);
        return;
    }
    let mut warn = [0u16; 4];
    let d = exe::date_sub_days(G_TRANSFER_RULE_CHANGE_DATE, warn.as_mut_ptr() as usize, 0x19);
    if read_u16(d + 2) == read_u16(G_CURRENT_DATE_YEAR) && read_i16(d) == read_i16(G_CURRENT_DATE) {
        covered!("sx11_1 transfer rules warning");
        exe::news_post_transfer_system_change(0);
    }
}

/// Builds a game date with the default `Date` (`exe::date_date_54a110`: day 40 of 1972).
pub(crate) unsafe fn construct_static_date(date: usize) {
    exe::date_date_54a110(date);
}

/// Comparer of a static date initialiser: the original runs (it registers the destructor once); the copy builds the date
/// into a local and its 8 bytes are compared with the global's.
pub(crate) unsafe fn compare_static_date(name: &str, original: unsafe extern "C" fn(), global: usize) {
    original();
    let mut copy = [0u16; 4];
    construct_static_date(copy.as_mut_ptr() as usize);
    let orig: [u16; 4] = core::array::from_fn(|k| read_u16(global + 2 * k));
    SCOREBOARD.record(copy == orig, || format!("{}: original={:x?} rust={:x?}", name, orig, copy));
}

/// `this` routine of the exe called with the game date (thiscall, ret 4).
pub(crate) unsafe fn start_date_call_today(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    g(this, G_CURRENT_DATE)
}

/// The daily update of every person: one day of a player's or a non-player's life (`staff_day_person`) for each
/// record of the staff table but the last (defect of the original: the end is base + (n - 1) × 0x6e with `<`, so the
/// last person never has the day); when a development moved some growth, the day's abilities are handed back in order
/// (`staff_day_hand_back_abilities`). The original builds an empty LLIST on its frame and destroys it at the end,
/// never using it: the copy has none. The favoured team mate's bonus starts at 0 (the original reads stack residue).
/// Defects of the original: the last person skipped; the uninitialised bonus; the gates `(id % 7) + day == 0` and the
/// like, which only open for id 0; an empty coach slot counted twice; in the ageing at 33 and 34, strength at 20 lost
/// two times in three.
/// Original: `game_daily_staff_update` (0x5ce2d0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_daily_staff_update() {
    let mut run = StaffDayRun { slot70: 0 };
    let mut changed = false;
    let table = read_usize(G_STAFF_TABLE);
    let end = table.wrapping_add((read_i32(G_STAFF_COUNT) as usize).wrapping_mul(staff::SIZE)).wrapping_sub(staff::SIZE);
    let mut p = table;
    while p < end {
        changed |= staff_day_person(p, &mut run);
        p += staff::SIZE;
    }
    if changed {
        staff_day_hand_back_abilities();
    }
}

/// Comparer for `game_daily_staff_update`: the original only (the day touches every person and posts news, offers and
/// shortlists through dozens of routines: it cannot run twice); the copy is proven by the emulation of the whole
/// routine (levas/fila/gra_2c/check_routine.py) and by the mutations through the career.
pub(crate) unsafe extern "C" fn compare_game_daily_staff_update() {
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_DAILY_STAFF_UPDATE)()
}

/// The end of the day, when some development moved an ability (0x5d587a..0x5d58e0): the player data records (the
/// array `G_PLAYER_DATA_PTRS_BY_ABILITY`) are sorted by current ability, then potential (qsort,
/// `compare_players_by_ability`), and the abilities sorted at the start of the day (`G_PLAYER_ABILITY_VALUES_SORTED`)
/// are handed back in that order, each capped at the record's potential: the spread of abilities stays the day's
/// [inference: what the original keeps by this].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d587a..0x5d58e0.
pub(crate) unsafe fn staff_day_hand_back_abilities() {
    exe::crt_qsort(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY), read_u32(G_PLAYER_DATA_COUNT), 4, FN_COMPARE_PLAYERS_BY_ABILITY);
    let mut i = 0usize;
    while (i as i32) < read_i32(G_PLAYER_DATA_COUNT) {
        let record = PlayerSheet(read_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i));
        record.set_current_ability((read_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i)) as i16);
        let potential = record.potential_ability();
        if record.current_ability() > potential {
            record.set_current_ability(potential);
        }
        i += 1;
    }
}

/// One person's day (0x5ce33c..0x5d5864). Skipped without a nation or once retired (state bits 0x30 = 0x20). The club
/// is the club of the contract found (`game_get_contract_and_store`), or the club job for the jobs 5 and 12; the club
/// job's linked B team is looked up (`club_get_linked_b_team`, 1); a club without a nation counts as none, else the
/// contract is the one with that club (`contract_manager_get_contract_for_club`). Without a player sheet the
/// non-player day (`staff_day_non_player`). A player wears (`Injury::daily_wear`) and has his day
/// (`staff_day_player`) on one of his three days of the year, while his contract's club plays in a loaded division
/// (`G_COMP_OBJECT_TABLE`), with current reputation 4750 or more, or when (id % 7) + today is 0; then, without a
/// club, the unattached decline; the training condition; on his three days of the year the ability tables, the
/// game's average (`game_average`), his turning age, the personality drift, the development and the normalisation
/// (`staff_normalise_player_attributes`, `staff_fade_staff_relationships`); and every day the shortlist check. Returns
/// whether a development moved the growth.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce33c..0x5d5864.
pub(crate) unsafe fn staff_day_person(p: usize, run: &mut StaffDayRun) -> bool {
    if read_usize(p + staff::NATION) == 0 {
        return false;
    }
    let id = read_i32(p);
    let state_off = (id as usize).wrapping_mul(staff_state::SIZE);
    if read_u8(read_usize(G_STAFF_STATE_TABLE) + state_off + staff_state::STATUS_FLAGS) & 0x30 == 0x20 {
        return false;
    }
    let pd = read_usize(p + staff::PLAYER);
    let found = Contract(game_get_contract_and_store(p));
    let mut club = if !found.is_null() { read_usize(G_CLUB_TABLE) + (found.club() as u32) as usize * club::SIZE } else { 0 };
    if club == 0 && matches!(read_u8(p + staff::JOB_FOR_CLUB), 5 | 12) {
        club = read_usize(p + staff::CLUB_JOB);
    }
    let mut b_team = 0u32;
    if read_usize(p + staff::CLUB_JOB) != 0 {
        exe::club_get_linked_b_team(read_usize(p + staff::CLUB_JOB), &mut b_team, 1);
    }
    let mut contract = 0;
    if club != 0 {
        if read_usize(club + club::NATION) != 0 {
            contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        } else {
            club = 0;
        }
    }
    if pd == 0 {
        staff_day_non_player(p, club, contract);
        return false;
    }
    let wear: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_INJURY_DAILY_WEAR_GRA);
    wear(read_usize(G_INJURY), id);
    let division = if club != 0 { read_usize(club + club::DIVISION) } else { 0 };
    let in_play = contract != 0
        && division != 0
        && read_usize(G_COMP_OBJECT_TABLE) != 0
        && read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(division) as usize) != 0;
    if !sd_is_development_day(id)
        && !in_play
        && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x128e
        && (id % 7).wrapping_add(read_i16(G_CURRENT_DATE) as i32) != 0
    {
        return false;
    }
    covered!("gra_2c player's day");
    let mut d = StaffDay { p, state_off, pd, club, contract, mate: 0, away_from_job: false, odds: 0, b_team };
    staff_day_player(&mut d, run);
    if club == 0 {
        staff_day_unattached_decline(p, pd);
    }
    staff_day_training_condition(p, club, contract);
    let mut changed = false;
    if sd_is_development_day(id) {
        staff_day_build_ability_tables();
        let average: unsafe extern "thiscall" fn(usize, u32) -> f64 = core::mem::transmute(FN_GAME_AVERAGE_GRA);
        let average = average(read_usize(G_PLAYER_STATS), 0);
        let sheet = read_usize(p + staff::PLAYER);
        if sheet == 0 {
            // dead: the player sheet was there at the start of the day
            show_error_at(GAME_CPP_PATH, 0x1901);
        } else {
            let club_job = read_usize(p + staff::CLUB_JOB);
            let age = read_i8(p + staff::AGE);
            let turning_age = staff_day_development_age(sheet);
            staff_day_personality_drift(p, age, club_job);
            let growth = development_day(p, sheet, club_job, age, turning_age, average);
            exe::staff_normalise_player_attributes(p);
            let fade: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_FADE_STAFF_RELATIONSHIPS_GRA);
            fade(p);
            changed = growth != 0;
        }
    }
    staff_day_shortlist_check(p, pd, club, contract);
    changed
}

/// The contracted player's day (0x5ce4b6..0x5d352d), piece by piece in the original's order.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce4b6..0x5d352d.
pub(crate) unsafe fn staff_day_player(d: &mut StaffDay, run: &mut StaffDayRun) {
    if staff_day_unrest_odds(d, run) {
        staff_day_grievance_unrest(d);
        staff_day_unrest_roll(d);
    }
    staff_day_disputes_and_morale(d);
    let contract = Contract(d.contract);
    if !contract.is_null() {
        staff_day_contract_moods(d);
        if contract.grievances() & 0x400000 != 0 && d.club == 0 {
            exe::staff_contract_clear_grievances(contract, 0x400000);
            staff_day_part_b(d);
            return;
        }
    }
    if d.club == 0 {
        staff_day_part_b(d);
        return;
    }
    let injury = exe::injury_get_player_injury(read_usize(G_INJURY), d.p);
    if injury != 0 && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), d.p) + 0x2c) > 30 {
        staff_day_part_b(d);
        return;
    }
    if contract.is_null() {
        return;
    }
    if !staff_day_grievance_fades(d) && read_usize(d.p + staff::PLAYER) != 0 && staff_day_youngster_praise(d).is_none() {
        if read_usize(d.p + staff::PLAYER) != 0 && staff_day_international_praise(d).is_none() {
            staff_day_club_stories(d);
        }
    }
    staff_day_value_demand(d);
    match staff_day_squad_status(d) {
        None => {
            exe::staff_contract_clear_grievances(contract, 0x10);
            exe::staff_contract_clear_grievances(contract, 2);
        }
        Some(true) => {}
        Some(false) => {
            if !staff_day_wants_move(d) {
                staff_day_contract_talks(d);
            }
        }
    }
    staff_day_part_b(d)
}

/// The day of a person without a player sheet (0x5d3367..0x5d352d). Without a non-player sheet nothing changes
/// (an unattached person goes through `staff_day_unattached_decline`, which then has nothing to do). With a
/// contract whose squad status (low nibble of `contract::SQUAD_STATUS`) is under 10, one draw in ftol(100 -
/// adaptability × 3.5) raises that nibble by one (the high nibble kept) and the person learns the language of the club's
/// nation (`nation::FIRST_LANGUAGE`): the first step adds it (`PlayerStats::add_language`), the next ones up to 10
/// improve it (`improve_language`), fluency 1, the list sorted. Unattached (no club): `staff_day_unattached_decline`.
/// At a club, except in the club job 5 [inference: the job byte `staff::JOB_FOR_CLUB`, 5 left alone]: the current
/// reputation becomes ftol(((club reputation + 50 × current ability) / 2) × 0.05 + current reputation × 0.95) (the
/// half truncated and taken to 16 bits); under 40 years old the current ability moves by -random(3) + random(6) three
/// days in four, else -random(6) + random(6); from 40 only one day in 20, by -random(6) + random(6) (the other days
/// keep it and skip the normalisation); capped at the potential and the attributes normalised
/// (`staff_normalise_non_player_attributes`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3367..0x5d352d.
pub(crate) unsafe fn staff_day_non_player(p: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let p = Staff(p);
    let club = Club(club);
    let np = p.non_player().addr();
    if np == 0 {
        if club.is_null() {
            staff_day_unattached_decline(p.addr(), 0);
        }
        return;
    }
    if !contract.is_null() && (contract.squad_status() & 0xf) < 10 {
        let n = ftol(read_f64(rdata::F64_100) - p.adaptability() as f64 * read_f64(rdata::F64_3_5));
        if exe::utils_random(n) == 0 {
            covered!("gra_2 non-player learns a language");
            let s = contract.squad_status();
            if s & 0xf < 10 {
                contract.set_squad_status(((s.wrapping_add(1) ^ s) & 0xf) ^ s);
            }
            let level = contract.squad_status() & 0xf;
            let language = read_u8(club.nation().addr() + nation::FIRST_LANGUAGE) as u32;
            if level == 1 {
                exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
            } else if level <= 10 {
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_IMPROVE_LANGUAGE);
                f(read_usize(G_PLAYER_STATS), p.addr(), language, 1, 1);
            }
        }
    }
    if club.is_null() {
        staff_day_unattached_decline(p.addr(), 0);
        return;
    }
    if p.job_for_club() == 5 {
        return;
    }
    let half = ((club.reputation() as i32).wrapping_add(50 * read_i16(np + non_player::CURRENT_ABILITY) as i32) / 2) as i16;
    let current = read_i16(np + non_player::CURRENT_REPUTATION);
    let r = ftol(half as f64 * read_f64(rdata::F64_0_05) + current as f64 * read_f64(rdata::F64_0_95));
    write_u16(np + non_player::CURRENT_REPUTATION, r as u16);
    let ability = np + non_player::CURRENT_ABILITY;
    if (p.age() as i8) < 40 {
        let down = if exe::utils_random(4) != 0 { exe::utils_random(3) } else { exe::utils_random(6) };
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    } else {
        if exe::utils_random(20) != 0 {
            return;
        }
        covered!("gra_2 non-player over 40 ability moves");
        let down = exe::utils_random(6);
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    }
    let potential = read_i16(np + non_player::POTENTIAL_ABILITY);
    if read_i16(ability) > potential {
        write_i16(ability, potential);
    }
    exe::staff_normalise_non_player_attributes(p);
}

/// An unattached person loses reputation (0x5d3539..0x5d377a; also the end of `staff_day_non_player`). The days
/// since joining (or leaving) the last club, (year - year joined) × 365 - day joined + today's day, against a grace of
/// 60 days plus 2 per cap (`staff::INT_APPS`): within it nothing. A player (`pd`, the player sheet) under 525 days loses
/// random(days - grace), halved (truncated, 16 bits) when the caps beat random(30); with a loss the current reputation
/// becomes the larger of 12.5 × current ability and current - loss, the home reputation the larger of 12.5 × current
/// ability and the new current - loss, the world reputation the larger of 5 × current ability and the new current -
/// loss. A non-player sheet of someone without a national job, between 361 and 524 days: loss ftol(random(days - 360)
/// × 0.25), drawn again as random(loss) when the nation's league is not loaded (`nation::LEAGUE_SELECTED`), halved when
/// the caps beat random(30); with a loss, half of it comes off the non-player current reputation (floor 35 × current
/// ability), the home reputation becomes the larger of 35 × current ability and the new current - half, the world
/// reputation the larger of 5 × current ability and the new current - half.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3539..0x5d377a.
pub(crate) unsafe fn staff_day_unattached_decline(p: usize, pd: usize) {
    let pd = PlayerSheet(pd);
    let p = Staff(p);
    let days = (read_u16(G_CURRENT_DATE_YEAR) as i32)
        .wrapping_sub(p.date_joined_club_year() as i32)
        .wrapping_mul(365)
        .wrapping_sub((p.date_joined_club() as i16) as i32)
        .wrapping_add(read_i16(G_CURRENT_DATE) as i32);
    let grace = p.int_apps() as i32 * 2 + 60;
    if days <= grace {
        return;
    }
    if !pd.is_null() && days < 525 {
        let mut loss = exe::utils_random(days - grace);
        if p.int_apps() as i32 > exe::utils_random(30) {
            loss = (loss as i16 as i32) / 2;
        }
        if (loss as i16) > 0 {
            covered!("gra_2 unattached player loses reputation");
            let loss = loss as i16 as i32;
            // fcom: the floor 12.5 x ability stays only when above the value (ties and NaN take the value)
            let floored = |x: i32| {
                let floor = pd.current_ability() as f64 * read_f64(rdata::F64_12_5);
                ftol(if floor > x as f64 { floor } else { x as f64 })
            };
            let current = floored((pd.current_reputation() as i32).wrapping_sub(loss));
            pd.set_current_reputation((current as u16) as i16);
            let home = floored((current as i16 as i32).wrapping_sub(loss));
            pd.set_home_reputation((home as u16) as i16);
            let x = (pd.current_reputation() as i32).wrapping_sub(loss);
            let five = pd.current_ability() as i32 * 5;
            pd.set_world_reputation((if five > x { five } else { x } as u16) as i16);
        }
    }
    let np = p.non_player().addr();
    if np == 0 || (p.national_job().addr() as u32) != 0 || days <= 360 || days >= 525 {
        return;
    }
    let mut loss = ftol(exe::utils_random(days - 360) as f64 * read_f64(rdata::F64_0_25));
    if read_u8(p.nation().addr() + nation::LEAGUE_SELECTED) == 0 {
        loss = exe::utils_random(loss as i16 as i32);
    }
    if p.int_apps() as i32 > exe::utils_random(30) {
        loss = (loss as i16 as i32) / 2;
    }
    if (loss as i16) <= 0 {
        return;
    }
    covered!("gra_2 unattached non-player loses reputation");
    let half = (loss as i16 as i32) / 2;
    let at_least = |floor: i32, x: i32| if floor > x { floor } else { x } as u16;
    let ability = |np: usize| read_i16(np + non_player::CURRENT_ABILITY) as i32;
    let current = |np: usize| read_i16(np + non_player::CURRENT_REPUTATION) as i32;
    write_u16(np + non_player::CURRENT_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::HOME_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::WORLD_REPUTATION, at_least(5 * ability(np), current(np).wrapping_sub(half)));
}

/// A player on a club's training schedule may come out of training below par (0x5d3786..0x5d38f9). The player's
/// schedule (`TRAINING_MANAGER::get_player_schedule`) must exist with its byte +0x15 set [inference: training today],
/// at a club, without a non-player sheet and not injured (`Injury::get_player_injury_status` other than 2). The odds
/// are ftol(professionalism³ + 30) (the game's pow), plus random(morale) with a contract, at least 20; one draw in them
/// (random below 1): with the injury object, no current injury (`Injury::get_player_injury`) and not AWOL, the match
/// condition (`Injury::get_match_condition`) drops by random(40) × 100 / (random(natural fitness), at least 1)
/// (`Injury::set_match_condition`, 16 bits); a drop of at least 1000 + random(750) leaving the condition at 8000 or
/// less, when the player is not away on international duty
/// (`NationalTeams::get_international_duty_club` with today's date), is reported (`AWOL::report_poor_fitness`, with the
/// club job).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3786..0x5d38f9.
pub(crate) unsafe fn staff_day_training_condition(p: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let p = Staff(p);
    let schedule = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
    if club == 0
        || schedule == 0
        || read_u8(schedule + 0x15) == 0
        || p.non_player().addr() != 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
    {
        return;
    }
    let mut odds = ftol(game_pow(p.professionalism() as f64, read_f64(rdata::F64_3)) + read_f64(rdata::F64_30));
    if !contract.is_null() {
        odds = odds.wrapping_add(exe::utils_random(contract.morale() as i32));
    }
    if odds < 20 {
        odds = 20;
    }
    if exe::utils_random(odds) >= 1 {
        return;
    }
    let injury = read_usize(G_INJURY);
    if injury == 0 || exe::injury_get_player_injury(injury, p) != 0 || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        return;
    }
    covered!("gra_2 tired after training");
    let pd = p.player();
    let fitness = exe::utils_random(pd.natural_fitness() as i32) as i8;
    let divisor = if fitness < 1 { 1 } else { fitness as i32 };
    let drop = exe::utils_random(40).wrapping_mul(100) / divisor;
    let get_condition: unsafe extern "thiscall" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_INJURY_GET_MATCH_CONDITION_GRA);
    let condition = get_condition(read_usize(G_INJURY), p.addr(), 0) as i16;
    let new = condition.wrapping_sub(drop as i16);
    exe::injury_set_match_condition(read_usize(G_INJURY), p, new, 0);
    let limit = exe::utils_random(750) + 1000;
    if (drop as i16 as i32) < limit || new > 8000 {
        return;
    }
    if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, 0, 0) != 0 {
        return;
    }
    covered!("gra_2 poor fitness reported");
    let report: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_AWOL_REPORT_POOR_FITNESS_GRA);
    report(read_usize(G_AWOL), p.addr(), p.club_job().addr());
}

/// The two tables of the day's ability redistribution, built by the day's first development (0x5d393e..0x5d3a0c):
/// while `G_ABILITY_TABLES_PENDING` is set, the current abilities of the whole player data table (+5 of each 0x46-byte
/// record) are copied to a new array (`crt_malloc`, 2 bytes each) and sorted (qsort, 16-bit compare 0x5d6200), and a
/// second array (4 bytes each) gets the address of each player data record in table order; the flag is cleared. A
/// failed allocation is the game.cpp error box at line 0x18d7 or 0x18e6 and the game ends (cleanup, exit(-1)). The
/// arrays are freed by whoever reads them [inference: not in this routine].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d393e..0x5d3a0c.
pub(crate) unsafe fn staff_day_build_ability_tables() {
    if read_u8(G_ABILITY_TABLES_PENDING) == 0 {
        return;
    }
    let values = exe::crt_malloc((read_i32(G_PLAYER_DATA_COUNT) as u32).wrapping_mul(2));
    write_usize(G_PLAYER_ABILITY_VALUES_SORTED, values);
    if values == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18d7);
    }
    let table = read_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i as usize, read_u16(table + 0x46 * i as usize + 5));
        i += 1;
    }
    exe::crt_qsort(values, read_u32(G_PLAYER_DATA_COUNT), 2, FN_COMPARE_ABILITY_VALUES);
    let pointers = exe::crt_malloc(read_u32(G_PLAYER_DATA_COUNT).wrapping_mul(4));
    write_usize(G_PLAYER_DATA_PTRS_BY_ABILITY, pointers);
    if pointers == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18e6);
    }
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i as usize, table + 0x46 * i as usize);
        i += 1;
    }
    write_u8(G_ABILITY_TABLES_PENDING, 0);
}

/// The age at which a player's development turns, by his best position (0x5d3a92..0x5d3af0): 31, or 28 with
/// attacker or attacking midfielder 15+, 29 with midfielder 15+, 30 with defensive midfielder 15+ (the first that
/// matches); then 33 for a goalkeeper 20, else 31 with sweeper 15+ or any defender rating. Defect of the original:
/// "defender != 0" (not "15+") sends nearly every outfield player back to 31, undoing the midfield and attack ages.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3a92..0x5d3af0.
pub(crate) unsafe fn staff_day_development_age(pd: usize) -> i8 {
    let pd = PlayerSheet(pd);
    let mut age = 31;
    if pd.attacker() >= 15 || pd.attacking_midfielder() >= 15 {
        age = 28;
    } else if pd.midfielder() >= 15 {
        age = 29;
    } else if pd.defensive_midfielder() >= 15 {
        age = 30;
    }
    if pd.goalkeeper() == 20 {
        age = 33;
    } else if pd.sweeper() >= 15 || pd.defender() != 0 {
        age = 31; // defect of the original: any defender rating
    }
    age
}

/// One of a player's three development days a year, one time in ten: his personality drifts (0x5d3af0..0x5d3d09).
/// Ambition +1 under 25 (below 20, one in three) or -1 over 25 (above 1, one in three). Then, each one time in four,
/// half up and half down (random(2)), between 1 and 20: loyalty (at 20 going up, the club job becomes a favoured club,
/// `staff_add_favoured_club` with random(2); at 1 going down, it stops being one, `staff_remove_favoured_club`),
/// determination, pressure (up by 2 for a player over 22 with professionalism above 10, pressure under 10 and current
/// reputation above 6750; else up by 1 only with professionalism above 5; down by 2 from 3 for a player under 21 with
/// current reputation above 5250, else down by 1 only with professionalism under 15), professionalism (up by 2 from
/// under 10 past 28 years old, else 1; down by 2 from 3 for a player under 23 at a club job whose reputation is below
/// his current reputation (above 4250), else 1), temperament and sportsmanship. `age` is the age byte, `club` the
/// club job (`staff::CLUB_JOB`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3af0..0x5d3d09.
pub(crate) unsafe fn staff_day_personality_drift(p: usize, age: i8, club: usize) {
    let p = Staff(p);
    let club = Club(club);
    use exe::utils_random as random;
    if random(10) != 0 {
        return;
    }
    covered!("gra_2 personality drift");
    let ambition = p.addr() + staff::AMBITION;
    if age < 25 {
        if read_i8(ambition) < 20 && random(3) == 0 {
            write_u8(ambition, read_u8(ambition).wrapping_add(1));
        }
    } else if age > 25 && read_i8(ambition) > 1 && random(3) == 0 {
        write_u8(ambition, read_u8(ambition).wrapping_sub(1));
    }
    // loyalty
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = p.loyalty();
        if up {
            if v < 20 {
                p.set_loyalty(v + 1);
            } else {
                let r = random(2);
                exe::staff_add_favoured_club(p, club, r as u32);
            }
        } else if v > 1 {
            p.set_loyalty(v - 1);
        } else {
            let remove: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_REMOVE_FAVOURED_CLUB_GRA);
            remove(p.addr(), club.addr());
        }
    }
    // determination
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = p.determination();
        if up {
            if v < 20 {
                p.set_determination(v + 1);
            }
        } else if v > 1 {
            p.set_determination(v - 1);
        }
    }
    // pressure
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = p.pressure();
        let pd = p.player().addr();
        let professionalism = p.professionalism();
        if up {
            if v < 20 {
                if pd != 0 && age > 22 && professionalism > 10 && v < 10 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                    p.set_pressure(v + 2);
                } else if professionalism > 5 {
                    p.set_pressure(v + 1);
                }
            }
        } else if v > 1 {
            if v > 2 && age < 21 && pd != 0 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1482 {
                p.set_pressure(v - 2);
            } else if professionalism < 15 {
                p.set_pressure(v - 1);
            }
        }
    }
    // professionalism
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = p.professionalism();
        if up {
            if v < 20 {
                p.set_professionalism(if v < 10 && age > 28 { v + 2 } else { v + 1 });
            }
        } else if v > 1 {
            let pd = p.player().addr();
            let fall_two = !club.is_null() && pd != 0 && v > 2 && age < 23 && {
                let reputation = read_i16(pd + player_sheet::CURRENT_REPUTATION);
                reputation > 0x109a && club.reputation() < reputation
            };
            p.set_professionalism(if fall_two { v - 2 } else { v - 1 });
        }
    }
    // temperament, sportsmanship
    for field in [staff::TEMPERAMENT, staff::SPORTSMANSHIP] {
        if random(4) == 0 {
            let up = random(2) == 0;
            let v = read_i8(p.addr() + field);
            if up {
                if v < 20 {
                    write_i8(p.addr() + field, v + 1);
                }
            } else if v > 1 {
                write_i8(p.addr() + field, v - 1);
            }
        }
    }
}

/// A player may join the clubs' shortlists (0x5d578d..0x5d5864, every day for every player, after the development).
/// Interest: current reputation above 5000 + random(3000), or caps above random(10) with world reputation above
/// 3000; then, if the player's state byte has no low nibble set (`G_STAFF_STATE_TABLE` +0xb) [inference: not a
/// retired or virtual player] and he has grievances (`contract::GRIEVANCES`) or is listed or asked to leave
/// (`staff_is_transfer_listed_or_requested`), one time in 17 he is added. Otherwise random(current reputation) above
/// random(2,000,000) adds him, and so does one time in 20 at a club without a division (`club::DIVISION` 0). Added:
/// `SHORTLIST_MANAGER::add_player_to_club_shortlists` (the transfer manager's shortlist manager, the player, 1).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d578d..0x5d5864.
pub(crate) unsafe fn staff_day_shortlist_check(p: usize, pd: usize, club: usize, contract: usize) {
    let pd = PlayerSheet(pd);
    let contract = Contract(contract);
    let p = Staff(p);
    let club = Club(club);
    use exe::utils_random as random;
    let interest = pd.current_reputation() as i32 > random(3000) + 5000
        || (p.int_apps() as i32 > random(10) && pd.world_reputation() > 3000);
    let add = 'decide: {
        if interest && read_u8(read_usize(G_STAFF_STATE_TABLE) + read_u32(p.addr()) as usize * 0x4f + 0xb) & 0xf == 0 {
            let unsettled = (!contract.is_null() && contract.grievances() != 0) || exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0;
            if unsettled && random(0x11) == 0 {
                covered!("gra_2 unsettled player shortlisted");
                break 'decide true;
            }
        }
        let a = random(pd.current_reputation() as i32);
        if a > random(2_000_000) {
            break 'decide true;
        }
        !club.is_null() && (club.division().addr() as u32) == 0 && random(0x14) == 0
    };
    if add {
        covered!("gra_2 player shortlisted");
        let manager = transfer_manager_get_shortlist_manager(read_usize(G_TRANSFER_MANAGER));
        exe::shortlist_manager_add_player_to_club_shortlists(manager, p, 1);
    }
}

/// One development day of a player (0x5d3d09..0x5d5771): the age factor, the form, the growth steps, fatigue and
/// injury, the training, the professionalism, the limited fall, the ability (held to the potential), the praise of a
/// big rise; the age factor x becomes random(ftol(x)) above -3; then the attributes of a declining (x < 0), strongly
/// developing (x > 1) or mildly developing player, anticipation and decisions for the developing ones, and the
/// ageing. Returns the day's growth (not 0: the day changed some ability).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3d09..0x5d5771.
pub(crate) unsafe fn development_day(p: usize, pd: usize, club: usize, age: i8, turning_age: i8, average: f64) -> i16 {
    covered!("gra_2c development day");
    let mut d = Development { p, pd, club, age, x: 0.0, mark: 0, injured: false, growth: 0, step: 0, saved_growth: 0 };
    development_age_factor(&mut d, turning_age);
    let (rating, s) = development_form(&mut d, average);
    development_growth_steps(&mut d, rating, s, average);
    development_fatigue_and_injury(&mut d);
    development_training(&mut d);
    development_professionalism(&mut d);
    development_fall_limit(&mut d);
    let before = development_ability(&mut d);
    development_praise(&d, before);
    let potential = read_i16(pd + player_sheet::POTENTIAL_ABILITY);
    if read_i16(pd + player_sheet::CURRENT_ABILITY) > potential {
        write_i16(pd + player_sheet::CURRENT_ABILITY, potential);
    }
    if d.x as f64 > read_f32(F32_NEG_3_DEVELOPMENT) as f64 {
        d.x = exe::utils_random(ftol(d.x as f64)) as f32;
    }
    if d.x < 0.0 {
        development_decline_attributes(&d);
    } else {
        if d.x as f64 > read_f64(rdata::F64_1) {
            development_strong_attributes(&mut d);
        } else {
            development_mild_attributes(&mut d);
        }
        development_reading_of_the_game(&d);
    }
    development_ageing(p, pd);
    d.growth
}

/// Today (`G_CURRENT_DATE`, 16 bits) is one of the person's three days of the year: id % 365 and the two 120 days
/// after it (each % 365, 16 bits).
pub(crate) unsafe fn sd_is_development_day(id: i32) -> bool {
    let first = (id % 365) as i16;
    let second = ((first as i32 + 120) % 365) as i16;
    let third = ((second as i32 + 120) % 365) as i16;
    let today = read_u32(G_CURRENT_DATE) as i16;
    today == first || today == second || today == third
}

/// 0x5d2392: the settling part needs both a contract and a club (without a club the caller runs the unattached decline).
pub(crate) unsafe fn staff_day_part_b(d: &StaffDay) {
    if d.contract != 0 && d.club != 0 {
        staff_day_settling(d);
    }
}

/// The start of a player's day (0x5ce4b6..0x5ce9f9): the favoured team mate, the fatigue and form unrest, the
/// youngster's squad status, and the odds of the day's unrest roll. Returns false when the day's unrest part is skipped
/// (the jump to 0x5cf391).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce4b6..0x5ce9f9.
pub(crate) unsafe fn staff_day_unrest_odds(d: &mut StaffDay, run: &mut StaffDayRun) -> bool {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let id = read_i32(p.addr());
    d.mate = sd_pick_favoured_team_mate(p.addr(), d.club);
    d.away_from_job = p.club_job().addr() != d.club;
    if read_u8(sd_unrest(d)) >= 50 {
        return false;
    }
    if random(200) == 0 {
        let limit = -1000 - random(5000);
        if (exe::injury_get_fatigue(read_usize(G_INJURY), id as u32) as i32) < limit {
            let unrest = sd_unrest(d);
            let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p.addr())) as f64;
            let n = random(ftol(read_f64(rdata::F64_20) - fatigue * read_f64(rdata::F64_0_02)));
            write_u8(unrest, read_u8(unrest).wrapping_add(n as u8));
        }
    }
    let contract = Contract(d.contract);
    if !contract.is_null() {
        if sd_grievances(d) & 0x100000 != 0 && random(p.pressure() as i32 + 5) == 0 {
            sd_add_unrest(d, random(5));
        }
        if d.club != 0 && (p.age() as i8) > 23 {
            let status = contract.squad_status();
            if status & 0xf0 == 0x60 || status & 0xf0 == 0x50 {
                if exe::human_manager_is_human_club(d.club, 0, 1, 0, 0) != 0 {
                    contract.set_squad_status(status & 0xf);
                } else {
                    contract.set_squad_status(contract.squad_status() & 0xf | 0x40);
                }
            }
        }
    }
    if read_u8(sd_unrest(d)) < 15 && random(20) == 0 && random(p.professionalism() as i32) == 0 {
        let form = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), d.pd) as i32;
        if form > random(20) + 35 {
            sd_add_unrest(d, random(20));
        }
    }
    let limit = ftol(read_i16(d.pd + player_sheet::CURRENT_ABILITY) as f64 * read_f64(rdata::F64_0_15)) as i8 as i32;
    if read_u8(sd_unrest(d)) as i32 >= limit {
        return false;
    }
    // The odds: by the gap between ability and unrest against the professionalism.
    let prof = p.professionalism() as i32;
    let loyalty = p.loyalty() as i32;
    let determination = p.determination() as i32;
    let pressure = p.pressure() as i32;
    let unsettled = !contract.is_null() && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && sd_grievances(d) & 0x30321 != 0;
    let gap = (read_i16(d.pd + player_sheet::CURRENT_ABILITY) as i32).wrapping_sub(read_u8(sd_unrest(d)) as i32);
    // The 16-bit sums of the original keep the gap's (and the quotient's) high half in the dword.
    let high = |x: i32| (x as u32 & 0xffff_0000) as i32;
    let low = |b: i32| b as i16 as u16 as i32;
    d.odds = if unsettled {
        if gap < prof + 80 {
            ftol((loyalty + prof * 2 + 10 + id % 20) as f64 * read_f64(rdata::F64_2_5))
        } else if gap > prof + 125 {
            (high(gap) | low(loyalty)).wrapping_add(id % 15).wrapping_add(high(id % 15) | low(prof)).wrapping_add(5)
        } else {
            let x = (prof * 2) as f64 + loyalty as f64 + (id % 20) as f64 + read_f32(rdata::F32_5) as f64;
            ftol(x + x)
        }
    } else if gap < prof + 80 {
        ftol((determination + prof + pressure + id % 20 + 10) as f64 * read_f64(rdata::F64_2_5))
    } else if gap > prof + 140 {
        (high(gap) | low(pressure))
            .wrapping_add(low(determination))
            .wrapping_add(id % 15)
            .wrapping_add(high(id / 15) | low(prof))
            .wrapping_add(5)
    } else {
        let x = (determination + prof + pressure + id % 20 + 5) as f64;
        ftol(x + x)
    };
    // The favoured team mate: a short the original never resets between people (`slot70`).
    if d.mate != 0 {
        let mut v = run.slot70 as i32;
        if !contract.is_null() && sd_grievances(d) & 0x20020 != 0 && read_u32(d.mate + staff::NATION) == (p.nation().addr() as u32) {
            v = v.wrapping_add(loyalty);
        } else {
            v = v.wrapping_add(ftol(loyalty as f64 * read_f64(rdata::F64_0_25)));
            if (v as i16) < 1 {
                v = 1;
            }
        }
        run.slot70 = v as u32;
        if (p.age() as i8) > 23 || read_i16(d.pd + player_sheet::CURRENT_REPUTATION) > 0x109a {
            v = random(v as i16 as i32);
            run.slot70 = v as u32;
        }
        d.odds = d.odds.wrapping_add(v);
    }
    if !contract.is_null() {
        if read_i16(d.pd + player_sheet::WORLD_REPUTATION) < 0xea6 && (p.age() as i8) < 19 {
            let age = p.age() as i8;
            let mut k = ftol(pressure as f64 * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_3)) as i8;
            if age < 17 {
                k = k.wrapping_sub(3);
            } else if age == 17 {
                k = k.wrapping_sub(1);
            }
            let club = Club(d.club);
            if !club.is_null() && club.nation().addr() != 0 {
                let nation = club.nation();
                let x = ((nation.league_standard() as i8) as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_0_6))
                    * club.reputation() as f64;
                // fcom: a NaN falls to the last step
                if x > read_f32(rdata::F32_7750) as f64 {
                    k = k.wrapping_sub(2);
                } else if x > read_f32(rdata::F32_5750) as f64 {
                    k = k.wrapping_sub(1); // 5750
                } else if !(x >= read_f32(rdata::F32_3750) as f64) {
                    k = k.wrapping_add(1); // 3750
                }
            }
            if k < 0 {
                k = 0;
            }
            if (contract.matches_left_out() as i8) > k {
                let mut n = 7i8.wrapping_sub(k);
                if n < 2 {
                    n = 2;
                }
                sd_add_unrest(d, random(n as i32));
            }
        }
        if sd_grievances(d) & 0x40000 != 0 {
            let pr = pressure as i16;
            if (d.odds as i16) > 25 - pr {
                d.odds = d.odds.wrapping_add(pr as i32 - 25);
                if (d.odds as i16) < 1 {
                    d.odds = 1;
                }
            }
        }
    }
    if read_i8(d.pd + player_sheet::GOALKEEPER) > 14 {
        d.odds = d.odds.wrapping_mul(2).wrapping_add(10);
    } else if read_i8(d.pd + player_sheet::DEFENDER) > 14 {
        d.odds = ftol(d.odds as i16 as f64 * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_5));
    }
    true
}

/// The grievances' unrest (0x5ce9f9..0x5cf106): one day in random(morale) for a player with grievances, each grievance
/// bit rolls random(contract morale + 200) against its own odds and adds random(5) to the unrest; two bits (0x20,
/// 0x20000) roll against (loyalty + 40) × 5 + morale when the favoured team mate shares the nation.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce9f9..0x5cf106.
pub(crate) unsafe fn staff_day_grievance_unrest(d: &StaffDay) {
    use exe::utils_random as random;
    let contract = d.contract;
    if contract == 0 || sd_grievances(d) == 0 || random(read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32) != 0 {
        return;
    }
    let mate_compatriot = |d: &StaffDay| d.mate != 0 && read_u32(d.mate + staff::NATION) == read_u32(d.p + staff::NATION);
    // the mate's roll: random((loyalty + 40) * 5 + morale) against random(100)
    let mate_roll = |d: &StaffDay| {
        let a = random((read_i8(d.p + staff::LOYALTY) as i32 + 40) * 5 + read_i8(d.contract + contract::MORALE) as i32);
        if a < random(100) {
            sd_add_unrest(d, random(5));
        }
    };
    if exe::staff_is_transfer_listed_or_requested(d.p, 0usize) == 0 && sd_grievances(d) & 1 != 0 {
        sd_grievance_roll(d, 20);
    }
    for (bit, odds) in [(2u32, 75), (8, 300)] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if sd_grievances(d) & 0x20 != 0 {
        if mate_compatriot(d) {
            mate_roll(d);
        } else {
            sd_grievance_roll(d, 200);
        }
    }
    for (bit, odds) in [
        (0x40u32, 100),
        (0x80, 40),
        (0x100, 50),
        (0x200, 100),
        (0x400, 20),
        (0x800, 20),
        (0x1000, 20),
        (0x2000, 400),
        (0x4000, 100),
        (0x8000, 50),
        (0x10000, 10),
    ] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if sd_grievances(d) & 0x20000 != 0 {
        if mate_compatriot(d) {
            mate_roll(d);
        } else {
            sd_grievance_roll(d, 100);
        }
    }
    for (bit, odds) in [(0x40000u32, 75), (0x80000, 75), (0x200000, 20), (0x400000, 20)] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if exe::staff_contract_get_flags(contract) & 8 != 0 {
        sd_grievance_roll(d, 20);
    }
    if exe::staff_contract_get_flags(contract) & 0x20 != 0 {
        sd_grievance_roll(d, 10);
    }
    if sd_grievances(d) & 0x1000000 != 0 {
        sd_grievance_roll(d, 20);
    }
    if sd_grievances(d) & 0x800000 != 0 && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), d.p, 0) == 0 {
        sd_grievance_roll(d, 100);
    }
}

/// The day's unrest roll of a club player (0x5cf106..0x5cf391): one time in random(odds), for the job 11 [inference:
/// a first team player], the unrest grows by random(8); a chairman losing patience may dislike him (rating draw over
/// his patience + 10 while the other rating is under 6.65, then one in patience - 2 × interference + 45); and one day
/// in 14 a human club's player with unrest above random(11), a rating above 1 and the other rating under the game's
/// average + unrest × 0.0133 gets a criticising training report from the assistant manager
/// (`news_post_training_report`), when the assistant judges well and is able enough.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf106..0x5cf391.
pub(crate) unsafe fn staff_day_unrest_roll(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    if p.job_for_club() != 11 || random(d.odds as i16 as i32) != 0 {
        return;
    }
    sd_add_unrest(d, random(8));
    let rating = sd_stat(p.addr(), 6, 1);
    let other = sd_stat(p.addr(), 6, 0x11);
    let club = Club(d.club);
    if club.is_null() {
        return;
    }
    let chairman = club.chairman();
    if !chairman.is_null() && chairman.non_player().addr() != 0 {
        let np = chairman.non_player().addr();
        let patience = read_i8(np + non_player::PATIENCE) as i16;
        if random(ftol(rating as f64)) > patience.wrapping_add(10) as i32
            && !(other as f64 >= read_f64(rdata::F64_6_65))
        // under 6.65 (or NaN)
        {
            let n = patience.wrapping_sub((read_i8(np + non_player::INTERFERENCE) as i16).wrapping_mul(2)).wrapping_add(45);
            if random(n as i32) == 0 {
                covered!("gra_2b chairman dislikes an unsettled player");
                exe::staff_add_disliked_staff(club.chairman().addr(), p, 1);
            }
        }
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        || club != p.club_job()
        || (read_i16(G_CURRENT_DATE) as i32 % 14).wrapping_add(read_i32(p.addr())) != 0
        || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
        || exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0
    {
        return;
    }
    let unrest = read_u8(sd_unrest(d)) as i32;
    if unrest <= random(11) || !(rating > read_f32(rdata::F32_1)) {
        return;
    }
    let bar = unrest as f64 * read_f64(rdata::F64_0_01333333333) + sd_average_rating();
    if !(bar > other as f64) {
        return;
    }
    let contract = Contract(d.contract);
    if contract.is_null() || contract.happiness() <= 0 {
        return;
    }
    let pd = p.player();
    if pd.is_null() || pd.current_reputation() <= 0xabe {
        return;
    }
    let assistant = Staff(club.assistant_manager());
    if assistant.is_null() || assistant.non_player().addr() == 0 {
        return;
    }
    let anp = assistant.non_player().addr();
    if exe::non_player_attribute_rating(anp, read_i8(anp + non_player::JUDGEMENT)) as i32 <= random(25) {
        return;
    }
    let r = random(400);
    if read_i16(read_usize(club.assistant_manager() + staff::NON_PLAYER) + non_player::CURRENT_ABILITY) as i32 <= r {
        return;
    }
    let schedule = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
    if schedule == 0 || read_u8(schedule + 0x15) == 0 {
        return;
    }
    covered!("gra_2b training report on an unsettled player");
    let f: unsafe extern "C" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_TRAINING_REPORT);
    f(p.addr(), club.assistant_manager(), club.addr(), 0);
}

/// The player's disputes, the fading unrest, the transfer status and the morale towards the club (0x5cf391..0x5cf57c).
/// A dispute ends without a club or with another club than the dispute's; a settled player at the club (no contract
/// flags, no grievances, happy) ends it with the news (`dispute_post_news`, outcome 3). The unrest of a fit player falls
/// by random(m / 5 + 2), m = random(morale + 5) when above random(40). A listed player at an AI club without grievances
/// who no longer wants to leave loses the transfer status bit 8 (unless his squad status is 0x70). One day in ten the
/// player's morale moves halfway to 10, or 10 + random(10 - the club's league position) when that is under 10, +3 when
/// the club favours him, -3 when it dislikes him (`staff_adjust_morale`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf391..0x5cf57c.
pub(crate) unsafe fn staff_day_disputes_and_morale(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if read_usize(G_DISPUTE) != 0 && exe::dispute_is_in_dispute(read_usize(G_DISPUTE), p, 0) != 0 {
        if club.is_null() {
            sd_remove_dispute(p);
        } else if read_i32(club.addr()) != exe::dispute_get_dispute_club_id(read_usize(G_DISPUTE), p) {
            sd_remove_dispute(p);
        } else if !contract.is_null()
            && exe::staff_contract_get_flags(contract) == 0
            && contract.grievances() == 0
            && contract.morale() > 0
        {
            covered!("gra_2b dispute settled");
            sd_remove_dispute(p);
            // stdcall in the map; the original also loads ecx with the dispute object, kept here
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_NEWS_GRA);
            f(read_usize(G_DISPUTE), p, club.addr(), 3, 0, 0);
        }
    }
    if read_u8(sd_unrest(d)) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2 {
        let m = random(read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 + 5) as i8 as i32;
        if m > random(40) {
            let mut fall = random(m / 5 + 2) as u8;
            let unrest = sd_unrest(d);
            let cur = read_u8(unrest);
            if fall > cur {
                fall = cur;
            }
            write_u8(unrest, cur - fall);
        }
    }
    if !contract.is_null()
        && !club.is_null()
        && contract.transfer_status() & 8 != 0
        && contract.grievances() == 0
        && exe::staff_contract_wants_to_leave(contract) == 0
        && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        && contract.squad_status() & 0xf0 != 0x70
    {
        contract.set_transfer_status(contract.transfer_status() & 0xf7);
    }
    if random(10) != 0 {
        return;
    }
    let mut target: u8 = 10;
    if !club.is_null() && club.division().addr() != 0 {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(club.division().addr()) as usize);
        if comp != 0 && read_usize(comp + 8) != 0 {
            let entry = exe::comp_stats_find_club_entry(read_usize(comp + 8), club);
            if entry != 0 && read_u8(entry + 4) < 10 {
                target = (random(10 - read_u8(entry + 4) as i32) as u8).wrapping_add(10);
            }
        }
    }
    if exe::club_favours_staff(p, club) != 0 {
        target = target.wrapping_add(3);
    } else if exe::club_dislikes_staff(p, club) != 0 {
        target = target.wrapping_sub(3);
    }
    let morale = read_i8(d.pd + player_sheet::PLAYER_MORALE);
    if morale != target as i8 {
        let half = (morale as i32 + target as i8 as i32) / 2;
        // `sub al, cl`: only the low byte of the half becomes the step
        let step = (half as u32 & 0xffff_ff00) | (half as u8).wrapping_sub(morale as u8) as u32;
        exe::staff_adjust_morale(p, step, 1);
    }
}

/// The contract flags and grievances that come and go (0x5cf57c..0x5cfa3a): the ambition and professionalism set a
/// threshold of the club's league standing (club state byte +4); flag 0x10 (wants a bigger club [inference]) is set
/// after loyalty + 90 days at a club below it, for a young squad status (0x10..0x30) with loyalty + professionalism
/// under 20, one time in professionalism + 5, and cleared when the club rises above it; flag 0x1000 fades one day in ten;
/// flag 0x200 fades when the matches left out beat random(by pressure) + 1; grievances 0x40000000 and 0x20000000 fade
/// one day in 7 by the club's standing, the ambition and the pressure (at once without a club); grievance 0x80000000
/// fades by the pressure, or one day in ten after more than two matches left out; a settled player gains 5 morale
/// (professional at a club without trouble, or loyal at a club standing above 2); flag 4 [inference: wants to play in
/// Europe] comes and goes by the club state byte +5.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf57c..0x5cfa3a.
pub(crate) unsafe fn staff_day_contract_moods(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = d.club;
    let contract = Contract(d.contract);
    let id = read_i32(p.addr());
    let day = read_i16(G_CURRENT_DATE) as i32;
    let ambition = p.ambition();
    let prof = p.professionalism();
    let mut bar: i8 = if ambition == 20 {
        -2
    } else if ambition > 15 {
        -3
    } else if ambition == 1 {
        -8
    } else {
        (ambition >= 6) as i8 - 5
    };
    if prof == 20 {
        bar -= 3;
    } else if prof > 15 {
        bar -= 2;
    } else if prof > 10 {
        bar -= 1;
    }
    if exe::staff_contract_get_flags(contract) & 0x10 != 0 {
        if club == 0 || sd_club_state(club, 4) > bar {
            exe::staff_contract_clear_flags(contract, 0x10);
        }
    } else if club != 0 {
        let loyalty = p.loyalty() as i32;
        let days = exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, read_u32(p.addr() + staff::DATE_JOINED_CLUB + 4));
        let status = contract.squad_status() & 0xf0;
        if days > loyalty + 90
            && sd_club_state(club, 4) < bar
            && (status == 0x10 || status == 0x20 || status == 0x30)
            && loyalty + (prof as i32) < 20
            && random(prof as i32 + 5) == 0
        {
            exe::staff_contract_set_flags(contract, 0x10);
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x1000 != 0 && random(10) == 0 {
        exe::staff_contract_clear_flags(contract, 0x1000);
    }
    let pressure = p.pressure();
    let calm: i8 = if pressure > 15 {
        0
    } else if pressure > 10 {
        1
    } else if pressure > 5 {
        2
    } else if pressure <= 1 {
        5
    } else {
        3
    };
    if exe::staff_contract_get_flags(contract) & 0x200 != 0 {
        let left_out = contract.matches_left_out() as i8;
        let r = random(calm as i32).wrapping_add(1);
        if left_out as i16 > r as i16 {
            exe::staff_contract_clear_flags(contract, 0x200);
        }
    }
    let pressure = pressure as i32;
    // grievances 0x40000000 and 0x20000000: the second rolls against the pressure, the first against pressure + 4
    for (bit, extra) in [(0x4000_0000u32, 4), (0x2000_0000, 0)] {
        if contract.grievances() & bit == 0 {
            continue;
        }
        if club == 0 {
            exe::staff_contract_clear_grievances(contract, bit);
            continue;
        }
        if day.wrapping_add(id) % 7 != 0 {
            continue;
        }
        let standing = sd_club_state(club, 4);
        let fade = if standing > 2 {
            random(pressure + extra) != 0
        } else {
            let a = random((ambition as i16).wrapping_add(if standing > 0 { 5 } else { 25 }) as i32);
            random(pressure + extra) > a
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    if contract.grievances() & 0x8000_0000 != 0 {
        let fade = if random(pressure) != 0 && random(15) == 0 {
            true
        } else {
            (contract.matches_left_out() as i8) > 2
                && (day % 10).wrapping_add(id) == 0
                && contract.happiness() > 0
                && random(pressure) != 0
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, 0x8000_0000);
        }
    }
    if club != 0 {
        if contract.grievances() != 0 {
            staff_day_european_flag(d);
            return;
        }
        if exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && sd_club_state(club, 5) == 0 && prof > 14 {
            staff_day_add_morale(contract.addr(), 5);
        }
    }
    if contract.grievances() == 0 && club != 0 && sd_club_state(club, 4) > 2 && p.loyalty() > 10 {
        staff_day_add_morale(contract.addr(), 5);
    }
    staff_day_european_flag(d);
}

/// The grievances of a player under contract at a club, fit enough, that fade or start (0x5cfa97..0x5d0041): an
/// unsettled player (reputation above 5750 with grievances, or transfer status bit 1) joins the clubs' shortlists one
/// day in 20; 0x80000 fades when random(pressure) beats random(100); 0x1000000 (wants a bigger wage [inference]) ends
/// without finances, within 30 days of the contract, below world reputation 5000, below ambition + 100 of ability or
/// past 32, and starts for a disloyal ambitious star attacker one day in 7 when 90% of the club's wage budget beats his
/// wage; 0x100000, 0x200000, 0x2000000 (with -5 morale while it lasts), 0x4000000 and the contract flags 0x40, 0x100
/// and 0x80 (-5 morale for a hot temper) fade by the ambition; a first team player out of the side, unhappy, ambitious,
/// impatient and famous may go to the media (`Media::create_player_media_story`, kind 0x23); flag 0x20 fades by the
/// sportsmanship; 0x8000000 costs 5 morale and fades by the loyalty. Returns true when grievance 0x40000 was there
/// (the routine then skips the media stories: 0x5d0e23), after costing 5 morale to the calm and fading by the pressure.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cfa97..0x5d0041.
pub(crate) unsafe fn staff_day_grievance_fades(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let pd = PlayerSheet(d.pd);
    let club = d.club;
    let contract = Contract(d.contract);
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let ambition = p.ambition() as i32;
    let state = read_usize(G_STAFF_STATE_TABLE) + read_u32(p.addr()) as usize * staff_state::SIZE;
    if read_u8(state + staff_state::STATUS_FLAGS) & 0xf == 0
        && ((pd.current_reputation() > 0x1676 && g(contract.addr()) != 0)
            || contract.transfer_status() & 1 != 0)
        && random(20) == 0
    {
        covered!("gra_2b unsettled player shortlisted");
        let listed = contract.transfer_status() & 1 != 0;
        let manager = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856); // the shortlist manager
        exe::shortlist_manager_add_player_to_club_shortlists(manager, p, listed as u32);
    }
    if g(contract.addr()) & 0x80000 != 0 {
        let a = random(p.pressure() as i32);
        if a > random(100) {
            exe::staff_contract_clear_grievances(contract, 0x80000);
        }
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let grievances = g(contract.addr());
    if grievances & 0x1000000 != 0 {
        let end = finance == 0
            || exe::date_days_since(G_CURRENT_DATE, contract.date_started() as u32, read_u32(contract.addr() + contract::DATE_STARTED + 4)) < 30
            || pd.world_reputation() < 5000
            || (pd.current_ability() as i32) < ambition + 100
            || (p.age() as i8) > 32;
        if end {
            exe::staff_contract_clear_grievances(contract, 0x1000000);
        }
    } else if p.loyalty() < 15
        && ambition > 15
        && (pd.attacker() == 20 || pd.attacking_midfielder() == 20)
        && grievances & 1 == 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && pd.current_ability() as i32 > ambition + 105
        && pd.world_reputation() > 0x1482
        && (read_i32(p.addr()) % 7).wrapping_add(read_i16(G_CURRENT_DATE) as i32) == 0
        && random(p.professionalism() as i32 + p.loyalty() as i32 + 10) == 0
        && ftol(read_i32(finance + 0x10) as f64 * read_f64(rdata::F64_0_9)) > contract.wage()
    {
        covered!("gra_2b star wants a bigger wage");
        exe::staff_contract_apply_event_flags(contract, 0x1000000, 0);
    }
    if g(contract.addr()) & 0x100000 != 0 {
        let fade = if (p.age() as i8) < 24 {
            contract.unhappy_flag() as i32 > 21 - p.pressure() as i32 && {
                let a = random(ambition);
                a > random(20)
            }
        } else {
            let r = random(4);
            contract.matches_left_out() as i32 >= r && {
                let a = random(ambition);
                a > random(100)
            }
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, 0x100000);
        }
    }
    if g(contract.addr()) & 0x200000 != 0 {
        let a = random(ambition);
        if a > random(50) {
            exe::staff_contract_clear_grievances(contract, 0x200000);
        }
    }
    if g(contract.addr()) & 0x2000000 != 0 {
        if random(ambition + 15) != 0 && contract.unhappy_flag() <= 1 {
            staff_day_add_morale(contract.addr(), -5);
        } else {
            exe::staff_contract_clear_grievances(contract, 0x2000000);
            if exe::staff_contract_get_flags(contract) & 0x40 != 0 {
                exe::staff_contract_clear_flags(contract, 0x40);
            }
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x40 != 0
        && (random(ambition + 5) == 0 || contract.unhappy_flag() > 0)
    {
        exe::staff_contract_clear_flags(contract, 0x40);
    }
    if exe::staff_contract_get_flags(contract) & 0x100 != 0 && random(ambition + 5) == 0 {
        exe::staff_contract_clear_flags(contract, 0x100);
    }
    if exe::staff_contract_get_flags(contract) & 0x80 != 0 {
        if random(ambition + 5) == 0 {
            exe::staff_contract_clear_flags(contract, 0x80);
        } else if p.temperament() < 6 {
            staff_day_add_morale(contract.addr(), -5);
        }
    }
    if g(contract.addr()) & 0x4000000 != 0
        && ((read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p.addr())) % 10 == 0 || random(ambition) == 0)
    {
        exe::staff_contract_clear_grievances(contract, 0x4000000);
    }
    if p.job_for_club() == 11
        && g(contract.addr()) & 0x10 != 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && !d.away_from_job
        && contract.matches_left_out() == 0
        && contract.unhappy_flag() > 1
    {
        let happiness = contract.happiness() as i32;
        let r = random(100);
        if happiness < (10 - p.loyalty() as i32) * 2 - r
            && ambition > random(10)
            && (p.professionalism() as i32) < random(5) + 5
            && (p.temperament() as i32) < random(5) + 5
            && {
                let r = random(3000) + 1500;
                read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) as i32 > r
            }
            && (p.age() as i8) > 19
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p.addr())).wrapping_add(8) % 14 == 0
            && random(p.professionalism() as i32) == 0
        {
            covered!("gra_2b player goes to the media");
            exe::media_create_player_media_story(read_usize(G_MEDIA), p, club, 0x23, 0, 0);
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x20 != 0 {
        let a = random(p.sportsmanship() as i32);
        if a >= random(50) {
            exe::staff_contract_clear_flags(contract, 0x20);
        }
    }
    if g(contract.addr()) & 0x8000000 != 0 {
        staff_day_add_morale(contract.addr(), -5);
        let a = random(p.loyalty() as i32);
        if a >= random(75) {
            exe::staff_contract_clear_grievances(contract, 0x8000000);
        }
    }
    if g(contract.addr()) & 0x40000 != 0 {
        if p.pressure() < 6 {
            staff_day_add_morale(contract.addr(), -5);
        }
        let a = random(p.pressure() as i32);
        if a >= random(50) {
            exe::staff_contract_clear_grievances(contract, 0x40000);
        }
        return true;
    }
    false
}

/// The youngster's praise (0x5d0041..0x5d02b5): a player under 22 without caps at a club in season with a manager,
/// fit, happy above random(40) - 20, at a club of reputation above 6000 + random(2000) or judged by the manager above
/// 130 + random(20) (`match_man_judge_player_ability`, rating (reputation / 2 + potential × 25)), one day in 14: with
/// under 100 games, a last rating of 7 or more, a season rating above 3 + random(5), form × 0.2 above the game's
/// average + 0.7 and the other rating above average + random(6) / 10, at a club standing above -2, gets a praise story
/// (topic 0x2b). Returns None when the youngster does not qualify (the routine goes on to the national praise).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0041..0x5d02b5.
pub(crate) unsafe fn staff_day_youngster_praise(d: &StaffDay) -> Option<()> {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if club.is_null()
        || club.manager().addr() == 0
        || d.away_from_job
        || p.int_apps() != 0
        || (p.age() as i8) >= 22
        || exe::club_is_in_season(club, G_CURRENT_DATE) == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 0
    {
        return None;
    }
    let happiness = contract.happiness() as i32;
    if happiness <= random(40) - 20 {
        return None;
    }
    let r = random(2000);
    if club.reputation() as i32 <= r + 6000 {
        let pd = p.player();
        let half = pd.current_reputation() as i32 / 2;
        // the 16-bit product, added to the half in 32 bits; the callee reads the short
        let rating = half.wrapping_add((pd.potential_ability()).wrapping_mul(25) as i32);
        let judged = exe::match_man_judge_player_ability(club.manager().addr(), p, club, 1, rating as i16) as i32;
        if judged <= random(20) + 130 {
            return None;
        }
    }
    let day = read_i16(G_CURRENT_DATE) as i32;
    if day.wrapping_add(read_i32(p.addr())).wrapping_add(5) % 14 != 9 {
        return None;
    }
    let average = sd_average_rating() as f32;
    let apps = exe::player_stats_get_apps_for_club(read_usize(G_PLAYER_STATS), p, club) as i16 as f32;
    let season = sd_stat(p.addr(), 6, 1);
    let other = sd_stat(p.addr(), 6, 0x11);
    let form = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), p.player().addr()) as f64 * read_f64(rdata::F64_0_2)) as f32;
    let last = sd_last_rating(p.addr());
    // the x87 tests: "under" holds for a NaN too, so each exit is the plain opposite
    if apps >= read_f32(rdata::F32_100) || !(last as f64 >= read_f64(rdata::F64_7)) {
        return Some(());
    }
    if (random(5) + 3) as f64 >= season as f64 {
        return Some(());
    }
    if (average as f64 + read_f64(rdata::F64_0_7)) >= form as f64 {
        return Some(());
    }
    let r = random(6);
    if (r as f64 * read_f64(rdata::F64_0_1) + average as f64) >= other as f64 {
        return Some(());
    }
    if sd_club_state(club.addr(), 4) <= -2 {
        return Some(());
    }
    covered!("gra_2b youngster praised");
    sd_praise_story(p.addr(), club.addr(), 0x2b);
    Some(())
}

/// The international's praise (0x5d02b5..0x5d0601): one day in 30 a happy international (happiness above random(40)),
/// fit, with fewer caps than 10 + random(10) or a world reputation under 5000 + random(2000), whose national team's
/// competition is a finals tournament (`NationalTeams::is_finals_comp`) starting between 10 and 100 days from today,
/// with a reputation above 5000 + random(3500), gets a praise story with the national team (topic 0x33) when his
/// season ratings beat 2 + random(4) and 5 + random(10) and both other ratings beat the average + 0.5 + random(6) / 10.
/// Returns None when he does not qualify (the club stories follow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d02b5..0x5d0601.
pub(crate) unsafe fn staff_day_international_praise(d: &StaffDay) -> Option<()> {
    use exe::utils_random as random;
    let p = Staff(d.p);
    if d.away_from_job || (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p.addr())).wrapping_add(5) % 30 != 13 {
        return None;
    }
    let happiness = read_i8(d.contract + contract::HAPPINESS) as i32;
    if happiness <= random(40)
        || p.national_job().addr() == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
    {
        return None;
    }
    let r = random(10) + 10;
    if p.int_apps() as i32 >= r {
        let r = random(2000) + 5000;
        if read_i16(p.player().addr() + player_sheet::WORLD_REPUTATION) as i32 >= r {
            return None;
        }
    }
    let teams = read_usize(G_NATIONAL_TEAMS);
    let comp = exe::nationalteams_get_current_competition(teams, read_usize(p.national_job().addr() + club::NATION));
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) == 0 {
        return None;
    }
    let r = random(3500) + 5000;
    if read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) <= r as i16 {
        return None;
    }
    let comp_object = |comp: usize| read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(comp) as usize);
    // the finals start after 10 days...
    let (day, year) = sd_date_in(10);
    let (first_day, first_year) = sd_first_date(comp_object(comp));
    if !(year < first_year || (year == first_year && day < first_day)) {
        return None;
    }
    // ...and within 100
    let (day, year) = sd_date_in(100);
    let (first_day, first_year) = sd_first_date(comp_object(comp));
    if !(year > first_year || (year == first_year && day > first_day)) {
        return None;
    }
    let average = sd_average_rating() as f32;
    let season = sd_stat(p.addr(), 6, 1);
    let season5 = sd_stat(p.addr(), 5, 1);
    let other = sd_stat(p.addr(), 6, 0x11);
    let other5 = sd_stat(p.addr(), 5, 0x11);
    if (random(4) + 2) as f64 >= season5 as f64 || (random(10) + 5) as f64 >= season as f64 {
        return Some(());
    }
    for rating in [other, other5] {
        let r = random(6);
        if (r as f64 * read_f64(rdata::F64_0_1) + average as f64 + read_f64(rdata::F64_0_5)) >= rating as f64 {
            return Some(());
        }
    }
    covered!("gra_2b international praised");
    sd_praise_story(p.addr(), p.national_job().addr(), 0x33);
    Some(())
}

/// The club's stories about a player in the side (0x5d0601..0x5d0e27): one day in 5, for a happy (above 45) player at
/// his club in season with a manager, fit and left out more than once, when random(professionalism / 2) is 0 or the
/// media already have a story on him. A first team player with good ratings at a club standing below 0 whose form
/// trails the average may criticise the club (`staff_day_club_criticism`); otherwise a player in form (last rating 7 or
/// more) may thank the manager who favours him (`news_post_player_thanks_manager`) or be praised (topic 0x2b).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0601..0x5d0e27.
pub(crate) unsafe fn staff_day_club_stories(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if p.player().addr() == 0 || d.away_from_job {
        return;
    }
    if club.is_null()
        || club.manager().addr() == 0
        || contract.happiness() <= 45
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        || (contract.matches_left_out() as i8) <= 1
        || (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p.addr())) % 5 != 3
        || exe::club_is_in_season(club, G_CURRENT_DATE) == 0
    {
        return;
    }
    if random(p.professionalism() as i32 / 2) != 0 && exe::media_has_story_for_player(read_usize(G_MEDIA), p) == 0 {
        return;
    }
    let average = sd_average_rating() as f32;
    let season = sd_stat(p.addr(), 6, 1);
    let other = sd_stat(p.addr(), 6, 0x11);
    let form = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), p.player().addr()) as f64 * read_f64(rdata::F64_0_2)) as f32;
    let floor = if season as f64 > read_f64(rdata::F64_1) { season as f64 } else { 1.0 };
    let ratio = (exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), p.player().addr(), 6, 2, 6.5) / floor) as f32;
    let s62 = sd_stat(p.addr(), 6, 2);
    let s65 = sd_stat(p.addr(), 6, 5);
    let last = sd_last_rating(p.addr());
    let standing = sd_club_state(club.addr(), 4);
    let mut bonus = standing as f64 * read_f64(rdata::F64_0_025);
    if bonus > read_f64(rdata::F64_0_2) {
        bonus = read_f32(rdata::F32_0_200000003_AT_96B028) as f64; // 0.2
    } else if !(bonus >= read_f64(rdata::F64_NEG_0_2)) {
        bonus = read_f32(rdata::F32_NEG_0_200000003_AT_96B024) as f64; // -0.2
    }
    let r = ClubStoryRatings { average, season, other: (bonus + other as f64) as f32, form, ratio, last };
    let avg = average as f64;
    if p.job_for_club() == 11
        && season > read_f32(rdata::F32_1)
        && avg - read_f64(rdata::F64_0_35) > bonus + form as f64
        && !(s62 >= read_f32(rdata::F32_10))
        && !(s65 >= read_f32(rdata::F32_10))
        && !(s65 as f64 + s62 as f64 >= read_f32(rdata::F32_15) as f64)
        && standing < 0
    {
        staff_day_club_criticism(d, &r);
        return;
    }
    staff_day_club_praise(d, &r, bonus);
}

/// The day's value and wage demand (0x5d0e27..0x5d10e5): a player who wants to leave, has morale under -25 or the
/// contract flag 8 gets a value (`CONTRACT_MANAGER::calc_player_value`) scaled by an unrest score (grievances,
/// ambition, loyalty, the club he likes or dislikes, youth) and the squad status at a big club; flag 8 (wants a better
/// contract [inference]) comes for a temperamental player valued above 250,000 and above the scaled value, and goes by
/// the professionalism.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0e27..0x5d10e5.
pub(crate) unsafe fn staff_day_value_demand(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if exe::staff_contract_wants_to_leave(contract) == 0
        && contract.morale() >= -25
        && exe::staff_contract_get_flags(contract) & 8 == 0
    {
        return;
    }
    let human = !club.is_null() && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    let value = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, human as u32);
    let g = contract.grievances();
    let mut score: i8 = if g != 0 { 5 } else { 0 };
    if g & 0x101 != 0 {
        score += 5;
    }
    let ambition = p.ambition();
    if ambition == 20 {
        score += 2;
    } else if ambition > 15 {
        score += 1;
    }
    let loyalty = p.loyalty();
    if loyalty == 1 {
        score += 3;
    } else if loyalty < 5 {
        score += 1;
    } else if loyalty == 20 {
        score -= 3;
    } else if loyalty > 15 {
        score -= 1;
    }
    if exe::staff_favours_club(p, club) != 0 {
        score -= 2;
    } else if exe::staff_dislikes_club(p, club) != 0 {
        score += 2;
    }
    let age = p.age() as i8;
    if age < 20 {
        score -= 4;
    } else if age < 23 {
        score -= 2;
    }
    let factor = read_f32(if score > 10 {
        rdata::F32_0_8999999762
    } else if score > 8 {
        rdata::F32_0_9499999881
    } else if score > 5 {
        rdata::F32_1
    } else if score > 3 {
        rdata::F32_1_100000024
    } else if score > 1 {
        rdata::F32_1_25
    } else if score == 0 {
        rdata::F32_1_75
    } else if score < -2 {
        rdata::F32_2_5
    } else if score < 0 {
        rdata::F32_2
    } else {
        rdata::F32_1_5
    }) as f64;
    let v = value as f64;
    let base = ftol(if value < 500_000 {
        v * read_f64(rdata::F64_4)
    } else if value < 2_500_000 {
        let x = v - read_f32(rdata::F32_500000) as f64;
        x + x + read_f64(rdata::F64_2000000)
    } else {
        (v - read_f32(rdata::F32_2500000) as f64) * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_6000000)
    });
    let mut demand = ftol(factor * base as f64);
    if !club.is_null() && club.reputation() > 0x1676 && contract.happiness() > 0 {
        let status = contract.squad_status() >> 4;
        let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
        let k = match status {
            1 => rdata::F64_1_2,
            2 if human => rdata::F64_1_15,
            2 => rdata::F64_1_2,
            3 if human => rdata::F64_1_1,
            3 => rdata::F64_1_15,
            _ if human => rdata::F64_1_05,
            _ => rdata::F64_1_1,
        };
        demand = ftol(demand as f64 * read_f64(k));
    }
    let worth = p.value() as i32;
    if exe::staff_contract_get_flags(contract) & 8 != 0 {
        if (worth < 250_000 || worth < demand) && random(p.professionalism() as i32) != 0 {
            exe::staff_contract_clear_flags(contract, 8);
        }
    } else if worth > 250_000 && worth > demand && random(p.temperament() as i32 + 5) == 0 {
        covered!("gra_2b player wants a better contract");
        exe::staff_contract_set_flags(contract, 8);
    }
}

/// The squad status and assurances (0x5d10e5..0x5d1448). Grievance 0x400000 ends without a club or at a club with a
/// wealth band of 0 or more. Nothing more for a player leaving on a Bosman, retiring, or in the jobs 6, 8, 9 and 12. A
/// player out for more than 14 days loses grievance 0x10 (the rest of the day's contract part is skipped). A player
/// without squad status at a club with a manager, not listed nor queued for a free transfer, seeks assurances when the
/// manager has been there long enough: (club reputation / player reputation)^6 (at most 6) × 50 + 10 (at most 120) +
/// id % 33 days; he loses 20 morale, and the human manager gets the news. A first team player whose squad status is
/// 0x70 [inference: not needed], over 21 (a goalkeeper over 24 + id % 4), one day in 20, loses 50 morale and complains
/// (grievance 0x10); a famous, ambitious, impatient one may then dislike the club or the manager. Returns false when
/// the routine goes on with the contract talks (0x5d1448), true when the day's contract part is over (0x5d2392).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d10e5..0x5d1448.
pub(crate) unsafe fn staff_day_squad_status(d: &StaffDay) -> Option<bool> {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if contract.grievances() & 0x400000 != 0 {
        let end = club.is_null() || {
            let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            finance != 0 && exe::finance_get_wealth_band(finance) as i8 >= 0
        };
        if end {
            exe::staff_contract_clear_grievances(contract, 0x400000);
        }
    }
    if contract.leaving_on_bosman() != 0 || exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) != 0 {
        return None;
    }
    let job = p.job_for_club();
    if job == 12 || job == 8 || job == 6 || job == 9 {
        return None;
    }
    if read_usize(G_INJURY) != 0
        && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        && exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
        && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), p) + 0x2c) > 14
    {
        exe::staff_contract_clear_grievances(contract, 0x10);
        return Some(true);
    }
    if contract.squad_status() & 0xf0 == 0
        && club.manager().addr() != 0
        && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) == 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
    {
        let reputation = read_i16(d.pd + player_sheet::CURRENT_REPUTATION);
        let denominator = if reputation < 1 { 1 } else { reputation as i32 };
        let mut x = game_pow(club.reputation() as f64 / denominator as f64, read_f64(rdata::F64_6));
        if x >= read_f64(rdata::F64_6) {
            x = read_f64(rdata::F64_6);
        }
        x = x * read_f64(rdata::F64_50) + read_f64(rdata::F64_10);
        if x >= read_f64(rdata::F64_120) {
            x = read_f64(rdata::F64_120);
        }
        let days = ftol((read_i32(p.addr()) % 33) as f64 + x);
        let mut out = [0u8; 8];
        let date = exe::date_add_days(club.manager().addr() + staff::DATE_JOINED_CLUB, out.as_mut_ptr() as usize, days as u32);
        let (day, year) = (read_i16(date), read_u16(date + 2));
        let later = year > read_u16(G_CURRENT_DATE_YEAR) || (year == read_u16(G_CURRENT_DATE_YEAR) && day > read_i16(G_CURRENT_DATE));
        if !later {
            covered!("gra_2b player seeks assurances");
            staff_day_add_morale(contract.addr(), -20);
            if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_SEEKS_ASSURANCES_NEWS);
                f(read_usize(G_CONTRACT_MANAGER), contract.addr());
            }
        }
        return Some(true);
    }
    let g = contract.grievances();
    if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0
        || job != 11
        || contract.squad_status() & 0xf0 != 0x70
        || g & 0x10 != 0
        || g & 4 != 0
        || random(20) != 0
        || p.player().addr() == 0
    {
        return Some(false);
    }
    let age = p.age() as i8;
    if age <= 21 {
        return Some(false);
    }
    if read_i8(p.player().addr() + player_sheet::GOALKEEPER) == 20 && age <= (read_i32(p.addr()) % 4 + 24) as i8 {
        return Some(false);
    }
    covered!("gra_2b unwanted player complains");
    staff_day_add_morale(contract.addr(), -50);
    exe::staff_contract_apply_event_flags(contract, 0x10, 0);
    let pd = p.player();
    if !pd.is_null()
        && pd.current_reputation() > 0x1482
        && p.ambition() > 5
        && p.temperament() < 15
        && p.sportsmanship() < 15
        && p.professionalism() < 10
        && random(4) != 0
    {
        sd_dislike_club_or_manager(d);
    }
    Some(true)
}

/// Wanting a move and settling (0x5d1448..0x5d16e8): one day in 50 a player without grievance 1 whose morale is low
/// (contract morale + 10 × (player morale - 10) under 25, or a first team international) and who has been 120 days at
/// the club (`sd_contract_days`) gets grievance 1 (wants a move, -50 morale, grievance 2 cleared) when his pull (at most
/// 9000) beats both the club's reputation and the reputation he sees in it (`club_reputation_for_staff`), under 31. With
/// grievance 1, one day in 50 (or every day past 31) the grievance ends with +50 morale when the reputation he sees
/// beats the pull (less 150 a year past 31) + 200 + random(400). Returns true when the day's contract part is over.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1448..0x5d16e8.
pub(crate) unsafe fn staff_day_wants_move(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if contract.grievances() & 1 == 0 {
        let low = contract.morale() as i32 + (read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 - 10) * 10;
        let eligible = low < 25 || (p.national_job().addr() != 0 && p.job_for_nation() == 11);
        if eligible && random(50) == 0 {
            let mut days = sd_days_since_inline(contract.addr() + contract::DATE_STARTED);
            if days < 0 {
                return true;
            }
            if days > 650 {
                days = sd_contract_days(p.addr(), club.addr(), 650);
            } else if days >= 120 {
                days = sd_contract_days(p.addr(), club.addr(), days);
            }
            if days < 250 {
                return true;
            }
            let mut pull = sd_pull(d, days);
            if pull > 9000 {
                pull = 9000;
            }
            if pull <= club.reputation() as i32
                || pull <= sd_club_reputation_for_staff(club.addr(), p.addr()) as i16 as i32
                || (p.age() as i8) >= 31
            {
                return true;
            }
            covered!("gra_2b player wants a move");
            staff_day_add_morale(contract.addr(), -50);
            exe::staff_contract_apply_event_flags(contract, 1, 0);
            exe::staff_contract_clear_grievances(contract, 2);
            return true;
        }
        if contract.grievances() & 1 == 0 {
            return false;
        }
    }
    let age = p.age() as i8;
    if random(50) != 0 && age <= 31 {
        return false;
    }
    let mut days = sd_days_since_inline(contract.addr() + contract::DATE_STARTED);
    if days < 0 {
        days = 0;
    } else if days > 650 {
        days = 650;
    }
    let mut pull = sd_pull(d, sd_contract_days(p.addr(), club.addr(), days));
    if age > 31 {
        pull += (31 - age as i32) * 150;
    }
    let seen = sd_club_reputation_for_staff(club.addr(), p.addr()) as i16 as i32;
    if random(400) + pull + 200 >= seen {
        return true;
    }
    covered!("gra_2b player settles");
    staff_day_add_morale(contract.addr(), 50);
    exe::staff_contract_clear_grievances(contract, 1);
    true
}

/// The contract talks (0x5d16e8..0x5d2392): grievance 2 (wants a new contract) ends within 60 days of the contract;
/// past loyalty × 2 + 5 days into the game and loyalty × 20 + 250 days into the contract, a settled player (no
/// grievance 1, not listed, squad status not 0x70, fit for 60 days, no transfer pending) one day in 50 weighs his
/// contract (`STAFF_CONTRACT::get_contract_satisfaction`) against the club's shortlist entry of him: the AI club offers a
/// new contract (`CONTRACT_MANAGER::offer_contract`) by the entry's status, else he asks for one (grievance 2) and may
/// go public (`CONTRACT_MANAGER::post_wants_new_contract_news`, disliking the club or manager); otherwise a loan player
/// near his contract's end wants to stay (grievance 2), and a veteran away from home may want to go back (grievance
/// 0x100), else the dressing room relations move his morale (`staff_day_relations`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d16e8..0x5d2392.
pub(crate) unsafe fn staff_day_contract_talks(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let contract = Contract(d.contract);
    if contract.grievances() & 2 != 0 && sd_days_since(contract.addr() + contract::DATE_STARTED) < 60 {
        exe::staff_contract_clear_grievances(contract, 2);
        return;
    }
    let loyalty = p.loyalty() as i16;
    let talks = sd_before_today(sd_add_days(G_GAME_START_DATE, loyalty.wrapping_mul(2).wrapping_add(5) as i32))
        && sd_before_today(sd_add_days(contract.addr() + contract::DATE_STARTED, loyalty.wrapping_mul(20).wrapping_add(250) as i32))
        && contract.grievances() & 1 == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && contract.squad_status() & 0xf0 != 0x70
        && random(50) == 0
        && !(read_usize(G_INJURY) != 0
            && exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
            && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), p) + 0x2c) > 60)
        && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
        && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) == 0;
    if talks {
        staff_day_new_contract(d);
    } else if !staff_day_contract_running_out(d) && !staff_day_going_home(d) {
        staff_day_relations(d);
    }
}

/// The age factor of a development day (0x5d3d09..0x5d3eba): x = turning age - age, × 0.25 when positive. Below 0
/// the player declines: when random(natural fitness) > 14, x becomes random(ftol(x × 10)) × 0.1; past 45 his three
/// reputations become 1; past 41 they are kept at 0.875, 0.875 and 0.75 (current, home, world); with x under -3 at
/// 0.925, 0.925 and 0.825; else at 0.975^-x, 0.975^-x and 0.95^-x (the game's pow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3d09..0x5d3eba.
pub(crate) unsafe fn development_age_factor(d: &mut Development, turning_age: i8) {
    use exe::utils_random as random;
    let pd = PlayerSheet(d.pd);
    let mut factor = (turning_age as i32 - d.age as i32) as f64;
    d.x = factor as f32;
    if factor > 0.0 {
        factor *= read_f64(rdata::F64_0_25);
        d.x = factor as f32;
    }
    if !(factor < 0.0) {
        return;
    }
    if random(pd.natural_fitness() as i32) > 14 {
        let r = random(ftol(d.x as f64 * read_f32(rdata::F32_10) as f64));
        d.x = (r as f64 * read_f32(rdata::F32_0_1000000015) as f64) as f32;
    }
    let keep = |field: usize, rate: f64| write_u16(pd.addr() + field, ftol(read_i16(pd.addr() + field) as f64 * rate) as u16);
    if d.age > 45 {
        covered!("gra_2c reputations of an old player drop to 1");
        pd.set_current_reputation(1u16 as i16);
        pd.set_home_reputation(1u16 as i16);
        pd.set_world_reputation(1u16 as i16);
    } else if d.age > 41 {
        keep(player_sheet::CURRENT_REPUTATION, read_f64(rdata::F64_0_875));
        keep(player_sheet::HOME_REPUTATION, read_f64(rdata::F64_0_875));
        keep(player_sheet::WORLD_REPUTATION, read_f64(rdata::F64_0_75));
    } else if (d.x as f64) < read_f32(F32_NEG_3_DEVELOPMENT) as f64 {
        keep(player_sheet::CURRENT_REPUTATION, read_f64(rdata::F64_0_925));
        keep(player_sheet::HOME_REPUTATION, read_f64(rdata::F64_0_925));
        keep(player_sheet::WORLD_REPUTATION, read_f64(rdata::F64_0_825));
    } else {
        let years = (-d.x) as f64;
        let rate = game_pow(read_f64(rdata::F64_0_975), years);
        keep(player_sheet::CURRENT_REPUTATION, rate);
        keep(player_sheet::HOME_REPUTATION, rate);
        keep(player_sheet::WORLD_REPUTATION, game_pow(read_f64(rdata::F64_0_95), years));
    }
}

/// The season's form behind a development day (0x5d3eba..0x5d4070). With the stats (`PlayerStats::get_stat`, 6.5 by
/// default) a = (6, 1), b = (6, 0x11), c = (1, 0x11) and d = (1, 1): t = (x × 25 + 50) × 0.01 × d, held to 0..100, and
/// s = t + a. The rating is the game's average - 0.25 when s is 0; else n = t × c + b × a, and n / s from s = 20 up,
/// below it ((n / s) × s + (20 - s) × the average) × 0.05. A developing player (x > 0) adds a × x × 0.01; his stamina
/// mark becomes 1 when random(10) + 5 is under t × 0.5 + a, and grows by one when a is not 0, his stamina is under 10
/// and his professionalism is above 5 and above random(26). Returns the rating and s (a float).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3eba..0x5d4070.
pub(crate) unsafe fn development_form(d: &mut Development, average: f64) -> (f64, f32) {
    use exe::utils_random as random;
    let a = sd_stat(d.p, 6, 1);
    let b = sd_stat(d.p, 6, 0x11);
    let c = sd_stat(d.p, 1, 0x11);
    let last = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), d.pd, 1, 1, 6.5);
    let t = (d.x as f64 * read_f64(rdata::F64_25) + read_f64(rdata::F64_50)) * read_f64(rdata::F64_0_01) * last;
    // fcomp: above 100 (or NaN) 100, below 0 nothing
    let t = if t.is_nan() || t > read_f64(rdata::F64_100) {
        read_f64(rdata::F64_100)
    } else if t < read_f64(rdata::F64_0) {
        read_f64(rdata::F64_0)
    } else {
        t
    };
    let part = t as f32;
    let s = t + a as f64;
    let mut rating = if s == 0.0 || s.is_nan() {
        average - read_f64(rdata::F64_0_25)
    } else {
        let n = part as f64 * c as f64 + b as f64 * a as f64;
        let twenty = read_f32(rdata::F32_20) as f64;
        if s < twenty {
            (n / s * s + (twenty - s) * average) * read_f64(rdata::F64_0_05)
        } else {
            n / s
        }
    };
    if d.x > 0.0 {
        let developed = (a as f64 * d.x as f64 * read_f64(rdata::F64_0_01) + rating) as f32;
        let threshold = (part as f64 * read_f32(rdata::F32_0_5) as f64 + a as f64) as f32;
        if ((random(10) + 5) as f64) < threshold as f64 {
            d.mark = 1;
        }
        let professionalism = read_i8(d.p + staff::PROFESSIONALISM);
        if a != 0.0 && !a.is_nan() && read_i8(d.pd + player_sheet::STAMINA) < 10 && professionalism > 5 && professionalism as i32 > random(0x1a) {
            d.mark = d.mark.wrapping_add(1);
        }
        rating = developed as f64;
    }
    (rating, s as f32)
}

/// The day's growth steps (0x5d4070..0x5d41f3). Under 22 the current and home reputations grow by 2.5% when s is
/// above 5, else by 1%. With r1 and r2 two draws of random(10), u = r1 × 0.05 - r2 × 0.05 (floats), v = (rating -
/// the average) × 4.5 and, for a developing player, w = potential² × 7.5e-5: the growth is the step
/// (`DEVELOPMENT_GROWTH_STEPS`) at ftol(w + u + v + x + 7), the physical step the one at ftol(u + x + 6.5) but at least
/// -8, both indexes held to 0..12.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4070..0x5d41f3.
pub(crate) unsafe fn development_growth_steps(d: &mut Development, rating: f64, s: f32, average: f64) {
    use exe::utils_random as random;
    let pd = PlayerSheet(d.pd);
    if d.age < 22 {
        let rate = if s as f64 > read_f32(rdata::F32_5) as f64 { read_f64(rdata::F64_1_025) } else { read_f64(rdata::F64_1_01) };
        pd.set_current_reputation((ftol(pd.current_reputation() as f64 * rate) as u16) as i16);
        pd.set_home_reputation((ftol(pd.home_reputation() as f64 * rate) as u16) as i16);
    }
    let v = ((rating - average) * read_f64(rdata::F64_4_5)) as f32;
    let first = (random(10) as f64 * read_f64(rdata::F64_0_05)) as f32;
    let u = (first as f64 - random(10) as f64 * read_f64(rdata::F64_0_05)) as f32;
    let w = if d.x as f64 > read_f64(rdata::F64_0) {
        sd_pow_double(pd.potential_ability() as f64, 2.0) * read_f64(F64_POTENTIAL_SQUARED_SCALE)
    } else {
        read_f32(rdata::F32_0) as f64
    };
    let growth_at = ftol(w + u as f64 + v as f64 + d.x as f64 + read_f64(rdata::F64_7));
    let step_at = ftol(u as f64 + d.x as f64 + read_f64(F64_6_5_DEVELOPMENT));
    let step = |at: i32| read_i16(DEVELOPMENT_GROWTH_STEPS + 2 * (at as i16).clamp(0, 12) as usize);
    d.growth = step(growth_at);
    d.step = step(step_at).max(-8);
}

/// Fatigue and injury against the growth (0x5d41f3..0x5d4384). With the fatigue (`Injury::get_fatigue`) under -500 -
/// random(1000) the stamina mark drops by random(3) past 28 years old, by random(2) past 24. Otherwise, with the
/// fatigue above 750 + random(750): a developing player whose fatigue is also above 1000 + random(200) loses
/// random(ftol(x × 25)) of growth, held at -8 under 19, -5 under 23, -3 under 30, else -20; and the mark drops by
/// random(3). A current injury whose days (+0x2c) give random(days) > 30 sets the growth to random(-5) and the injury
/// flag; one time in ten the mark drops by random(2).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d41f3..0x5d4384.
pub(crate) unsafe fn development_fatigue_and_injury(d: &mut Development) {
    use exe::utils_random as random;
    let id = read_u32(d.p);
    let fatigue = || exe::injury_get_fatigue(read_usize(G_INJURY), id) as i32;
    let limit = -500 - random(1000);
    if fatigue() < limit {
        if d.age > 28 {
            d.mark = d.mark.wrapping_sub(random(3) as i8);
        } else if d.age > 24 {
            d.mark = d.mark.wrapping_sub(random(2) as i8);
        }
    } else {
        let tired = fatigue();
        if tired > random(750) + 750 {
            if d.x > 0.0 {
                let very_tired = fatigue();
                if very_tired > random(200) + 1000 {
                    covered!("gra_2c tired youngster loses growth");
                    let cut = random(ftol(d.x as f64 * read_f32(rdata::F32_25) as f64));
                    d.growth = d.growth.wrapping_sub(cut as i16);
                    let floor = if d.age < 19 {
                        -8
                    } else if d.age < 23 {
                        -5
                    } else if d.age < 30 {
                        -3
                    } else {
                        -20
                    };
                    if d.growth < floor {
                        d.growth = floor;
                    }
                }
            }
            d.mark = d.mark.wrapping_sub(random(3) as i8);
        }
    }
    let injury = read_usize(G_INJURY);
    if injury == 0 || exe::injury_get_player_injury(injury, d.p) == 0 {
        return;
    }
    let days = read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), d.p) + 0x2c);
    if random(days as i32) > 30 {
        covered!("gra_2c injury cuts the growth");
        d.growth = random(-5) as i16;
        if random(10) == 0 {
            d.mark = d.mark.wrapping_sub(random(2) as i8);
        }
        d.injured = true;
    }
}

/// The training behind a positive growth (0x5d4384..0x5d46b1). Without a club job the growth is ftol(growth × 0.2).
/// At a club: the factor starts at ((training facilities / 26.6, or reputation / 13,300 without them) + 0.75) × 0.5;
/// a computer manager with a non-player sheet multiplies it by youngsters / 26.6 + 0.75, else it is × 1.31. Then the
/// assistant and the five coaches (an empty coach slot repeats the last coach seen: defect of the original), each with
/// a non-player sheet, multiply it by w / 53.2 + 0.875, w being (coach rating + 2 × youngsters) × 0.33 under 20, (2 ×
/// coach rating + youngsters) × 0.33 under 23, else the coach rating. The level (division reputation / 30.7, or club
/// reputation / 15,350 without a division, + 0.5, at most 1) multiplies it too; one time in five it is 1; held to
/// 0.1..3. The growth becomes ftol(growth × factor), within 5 of the old one, and 2..25.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4384..0x5d46b1.
pub(crate) unsafe fn development_training(d: &mut Development) {
    use exe::utils_random as random;
    if d.growth <= 0 {
        return;
    }
    let club = Club(d.club);
    if club.is_null() {
        d.growth = ftol(d.growth as f64 * read_f64(rdata::F64_0_2)) as i16;
        return;
    }
    covered!("gra_2c training factor");
    let facilities = if (club.training() as u8) != 0 {
        club.training() as f64 * read_f64(F64_TRAINING_SCALE)
    } else {
        club.reputation() as f64 * read_f64(F64_CLUB_REPUTATION_TRAINING_SCALE)
    };
    let mut factor = ((facilities + read_f64(rdata::F64_0_75)) * read_f32(rdata::F32_0_5) as f64) as f32;
    let manager = club.manager();
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && !manager.is_null() && manager.non_player().addr() != 0 {
        let youngsters = read_i8(manager.non_player().addr() + non_player::YOUNGSTERS);
        factor = ((youngsters as f64 * read_f64(F64_TRAINING_SCALE) + read_f64(rdata::F64_0_75)) * factor as f64) as f32;
    } else {
        factor = (factor as f64 * read_f32(F32_1_31_HUMAN_MANAGER) as f64) as f32;
    }
    let mut coach = 0usize;
    for slot in -1i32..5 {
        if slot < 0 {
            coach = club.assistant_manager();
        } else {
            let next = club.coaches(slot as usize);
            if next != 0 {
                coach = next;
            }
        }
        if coach == 0 || read_usize(coach + staff::NON_PLAYER) == 0 {
            continue;
        }
        let np = read_usize(coach + staff::NON_PLAYER);
        let rating = development_coach_rating(d.p, np) as i32;
        let youngsters = read_i8(np + non_player::YOUNGSTERS) as i32;
        let w = if d.age < 20 {
            (rating + youngsters * 2) as f64 * read_f64(rdata::F64_0_33)
        } else if d.age < 23 {
            (youngsters + rating * 2) as f64 * read_f64(rdata::F64_0_33)
        } else {
            rating as f64
        };
        factor = ((w * read_f64(F64_COACH_SCALE) + read_f64(rdata::F64_0_875)) * factor as f64) as f32;
    }
    let division = club.division().addr();
    let level = if division != 0 {
        read_i16(division + competition::REPUTATION) as f64 * read_f64(F64_DIVISION_LEVEL_SCALE)
    } else {
        club.reputation() as f64 * read_f64(F64_CLUB_LEVEL_SCALE)
    } + read_f64(rdata::F64_0_5);
    let level = if level > read_f64(rdata::F64_1) { read_f32(rdata::F32_1) as f64 } else { level };
    let mut factor = (level * factor as f64) as f32;
    if random(5) == 0 {
        factor = 1.0;
    }
    let factor = if !(factor as f64 >= read_f64(rdata::F64_0_1)) {
        read_f64(rdata::F64_0_1)
    } else if factor as f64 > read_f64(rdata::F64_3) {
        read_f64(rdata::F64_3)
    } else {
        factor as f64
    };
    let growth = d.growth as i32;
    let mut trained = ftol(growth as f64 * factor) as i16 as i32;
    if growth - 5 > trained {
        trained = growth - 5;
    } else if growth + 5 < trained {
        trained = growth + 5;
    }
    d.growth = (trained as i16).clamp(2, 25);
}

/// The professionalism against the growth (0x5d46b1..0x5d4800), with k = id % 4 (C remainder): under 24 + k years old a
/// negative growth becomes random(growth) unless random(professionalism) is 0; else under 30 with professionalism above
/// 10 and random(professionalism) above random(200), the growth becomes at least random(20) + growth (low bytes added),
/// at most 25; else past 26 + k with professionalism under 6 and random(professionalism + 6) twice 0, the growth
/// becomes at most growth - random(20) (low bytes), at least -25.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d46b1..0x5d4800.
pub(crate) unsafe fn development_professionalism(d: &mut Development) {
    use exe::utils_random as random;
    let k = read_i32(d.p) % 4;
    let age = d.age as i32;
    let professionalism = || read_i8(d.p + staff::PROFESSIONALISM) as i32;
    if age < 24 + k && d.growth < 0 && random(professionalism()) != 0 {
        d.growth = random(d.growth as i32) as i16;
        return;
    }
    if d.age < 30 && professionalism() > 10 && random(professionalism()) > random(200) {
        covered!("gra_2c professional player grows more");
        let sum = (random(20) as u8).wrapping_add(d.growth as u8) as i8;
        if d.growth as i32 <= (sum as i32).min(25) {
            d.growth = (sum as i16).min(25);
        }
        return;
    }
    if age > 26 + k && professionalism() < 6 && random(professionalism() + 6) == 0 && random(professionalism() + 6) == 0 {
        covered!("gra_2c unprofessional veteran grows less");
        let difference = (d.growth as u8).wrapping_sub(random(20) as u8) as i8;
        if d.growth as i32 >= (difference as i32).max(-25) {
            d.growth = (difference as i16).max(-25);
        }
    }
}

/// A tired young player's fall is limited (0x5d4800..0x5d488d): under 30, without the injury flag, at a club job, with
/// the fatigue above 1050 and a negative growth whose size passes 5% of the current ability, the growth becomes
/// ftol(-ability × 0.05), at most -1.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4800..0x5d488d.
pub(crate) unsafe fn development_fall_limit(d: &mut Development) {
    if d.age >= 30 || d.injured || d.club == 0 || read_usize(G_INJURY) == 0 {
        return;
    }
    if exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(d.p)) <= 1050 || d.growth >= 0 {
        return;
    }
    let ability = read_i16(d.pd + player_sheet::CURRENT_ABILITY) as i32;
    if ability as f64 * read_f64(rdata::F64_0_05) < d.growth as f64 * read_f64(rdata::F64_NEG_1) {
        covered!("gra_2c tired player's fall limited");
        d.growth = (ftol(-ability as f64 * read_f64(rdata::F64_0_05)) as i16).min(-1);
    }
}

/// The current ability moves (0x5d488d..0x5d4996): it gains random(growth) (16 bits). At a club job in a division
/// whose reputation × 500 + the club's reputation is under 12,000, a rise is kept unless random(5) is 0; otherwise,
/// above the ceiling ftol((division reputation² × 25 + club reputation) × 0.01) (the game's pow) the ability becomes
/// ftol(ceiling × 0.1) - ftol(ability × -0.9) (16 bits), never below the ability of before. Returns the ability of
/// before.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d488d..0x5d4996.
pub(crate) unsafe fn development_ability(d: &mut Development) -> i16 {
    use exe::utils_random as random;
    let ability = d.pd + player_sheet::CURRENT_ABILITY;
    d.saved_growth = d.growth as i32;
    let before = read_i16(ability);
    let rise = random(d.growth as i32);
    write_u16(ability, read_u16(ability).wrapping_add(rise as u16));
    let club = Club(d.club);
    if club.is_null() {
        return before;
    }
    let division = club.division().addr();
    if division == 0 {
        return before;
    }
    let division_reputation = read_i16(division + competition::REPUTATION) as i32;
    if (club.reputation() as i32).wrapping_add(division_reputation.wrapping_mul(500)) >= 12000
        || read_i16(ability) <= before
        || random(5) == 0
    {
        return before;
    }
    let ceiling = ftol(
        (game_pow(division_reputation as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_25) + club.reputation() as f64)
            * read_f64(rdata::F64_0_01),
    ) as i16;
    if read_i16(ability) > ceiling {
        covered!("gra_2c weak division caps the rise");
        let kept = ftol(read_i16(ability) as f64 * read_f64(F64_NEG_0_9_DEVELOPMENT)) as i16;
        let capped = (ftol(ceiling as f64 * read_f64(rdata::F64_0_1)) as i16).wrapping_sub(kept);
        write_i16(ability, if capped < before { before } else { capped });
    }
    before
}

/// A big rise is praised (0x5d4996..0x5d4ad2): when the ability rose by more than random(20) to above 80 at a club job
/// run by a human manager (`news_club_has_human_manager`), with the unrest counter under 10, an assistant, the injury
/// object, no injury, not AWOL, and a first team contract (type & 0x3f = 2) with the happiness below 0 or the unhappy
/// count above 5: when the selection score (`match_man_score_player_for_selection`: player, -1, 0x41e88, club, -1,
/// assistant) passes 85% of the club's target rating (`player_search_get_club_target_rating`), the assistant's
/// training report praises him (`news_post_training_report`, 1).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4996..0x5d4ad2.
pub(crate) unsafe fn development_praise(d: &Development, before: i16) {
    let (p, club) = (d.p, d.club);
    let ability = read_i16(d.pd + player_sheet::CURRENT_ABILITY);
    if ability as i32 - before as i32 <= exe::utils_random(20) || ability <= 80 || club == 0 {
        return;
    }
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) == 0
        || read_u8(read_usize(G_STAFF_STATE_TABLE) + (read_u32(p) as usize).wrapping_mul(staff_state::SIZE) + staff_state::INJURY_COUNTER) >= 10
        || read_usize(club + club::ASSISTANT_MANAGER) == 0
        || read_usize(G_INJURY) == 0
        || exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
        || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
    {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
    if contract.is_null() || contract.contract_type() & 0x3f != 2 {
        return;
    }
    if contract.happiness() >= 0 && contract.unhappy_flag() <= 5 {
        return;
    }
    let score = exe::match_man_score_player_for_selection(p, u32::MAX, 0x41e88, club, -1, read_usize(club + club::ASSISTANT_MANAGER));
    let target_rating: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_PLAYER_SEARCH_GET_CLUB_TARGET_RATING_GRA);
    let target = target_rating(club, 0, 0);
    if (target as f64 * read_f64(rdata::F64_0_85)) < score as f64 {
        covered!("gra_2c training report praises a big rise");
        let f: unsafe extern "C" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_TRAINING_REPORT);
        f(p, read_usize(club + club::ASSISTANT_MANAGER), club, 1);
    }
}

/// A declining player's physical attributes (0x5d4b24..0x5d4daf), with the step a and the divisor q = (natural
/// fitness × 0.4 + 4) × 50 × 0.01, at least 1: acceleration, agility, jumping, natural fitness, pace, stamina (plus the
/// stamina mark) and strength each move by ftol(random(a) / q), held to 1..20; reflexes by random(saved growth), held
/// to -125..125; vision between 1 and 99 becomes the larger of × 1.1 and + 5; consistency moves by random(2) when
/// random(professionalism) beats random(50); positioning under 100 grows like vision when the defender rating beats
/// random(5) + 15.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4b24..0x5d4daf.
pub(crate) unsafe fn development_decline_attributes(d: &Development) {
    use exe::utils_random as random;
    let pd = PlayerSheet(d.pd);
    let q = (pd.natural_fitness() as f64 * read_f64(rdata::F64_0_4) + read_f64(rdata::F64_4))
        * read_f64(rdata::F64_50)
        * read_f64(rdata::F64_0_01);
    let divisor = if q < read_f64(rdata::F64_1) { 1.0f32 } else { q as f32 };
    let step = d.step as i32;
    for field in [player_sheet::ACCELERATION, player_sheet::AGILITY, player_sheet::JUMPING, player_sheet::NATURAL_FITNESS, player_sheet::PLAYER_PACE] {
        let r = random(step);
        sd_clamp_attribute(pd.addr(), field, ftol(r as f64 / divisor as f64), 1, 20);
    }
    let r = random(step);
    sd_clamp_attribute(pd.addr(), player_sheet::STAMINA, ftol(r as f64 / divisor as f64 + d.mark as i32 as f64), 1, 20);
    let r = random(step);
    sd_clamp_attribute(pd.addr(), player_sheet::STRENGTH, ftol(r as f64 / divisor as f64), 1, 20);
    sd_clamp_attribute(pd.addr(), player_sheet::REFLEXES, random(d.saved_growth), -0x7d, 0x7d);
    sd_grow_ten_percent(pd.addr(), player_sheet::VISION, true);
    if random(read_i8(d.p + staff::PROFESSIONALISM) as i32) > random(50) {
        sd_clamp_attribute(pd.addr(), player_sheet::CONSISTENCY, random(2), 1, 20);
    }
    if pd.defender() as i32 > random(5) + 15 {
        sd_grow_ten_percent(pd.addr(), player_sheet::POSITIONING, false);
    }
}

/// A strongly developing player's mental attributes (0x5d4dc9..0x5d4f5e, x above 1), with the step a: consistency,
/// versatility and important matches move by random(3a) / 10 and leadership by random((random(2) + 2) × a) / 10, held
/// to 1..20. The stamina mark grows by one when the training entry's level (signed low nibble of byte 3,
/// `training_manager_get_player_entry`) beats random(4) + 3, and a negative mark is cleared at a club job whose training
/// facilities beat random(31). Stamina (only while under natural fitness + 5) and strength move by random(3a) / ((5 -
/// mark) × 2).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4dc9..0x5d4f5e.
pub(crate) unsafe fn development_strong_attributes(d: &mut Development) {
    use exe::utils_random as random;
    let pd = PlayerSheet(d.pd);
    let step = d.step as i32;
    let three = step * 3;
    sd_clamp_attribute(pd.addr(), player_sheet::CONSISTENCY, random(three) / 10, 1, 20);
    let lead = random((random(2) + 2).wrapping_mul(step));
    sd_clamp_attribute(pd.addr(), player_sheet::LEADERSHIP, lead / 10, 1, 20);
    sd_clamp_attribute(pd.addr(), player_sheet::VERSATILITY, random(three) / 10, 1, 20);
    sd_clamp_attribute(pd.addr(), player_sheet::IMPORTANT_MATCHES, random(three) / 10, 1, 20);
    let entry_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_ENTRY_GRA);
    let entry = entry_of(read_usize(G_TRAINING_MANAGER), d.p);
    let level = ((read_u8(entry + 3) << 4) as i8 >> 4) as i32;
    if level > random(4) + 3 {
        d.mark = d.mark.wrapping_add(1);
    }
    if d.club != 0 && read_i8(d.club + club::TRAINING) as i32 > random(0x1f) && d.mark < 0 {
        d.mark = 0;
    }
    // (5 - mark) x 2 is never 0: the mark stays far below 5 here
    if (pd.stamina() as i32) < pd.natural_fitness() as i32 + 5 {
        let r = random(three);
        sd_clamp_attribute(pd.addr(), player_sheet::STAMINA, r / ((5 - d.mark as i32) * 2), 1, 20);
    }
    let r = random(three);
    sd_clamp_attribute(pd.addr(), player_sheet::STRENGTH, r / ((5 - d.mark as i32) * 2), 1, 20);
}

/// A mildly developing player (0x5d4f63..0x5d50ab, x from 0 to 1). A negative stamina mark is cleared when the natural
/// fitness passes stamina + 2 or random(natural fitness) beats random(20); else it gains random(mark + 1). A positive
/// mark with stamina above 15 becomes random(mark). The stamina moves by the mark (1..20); above natural fitness + 5 it
/// loses random(2) when random(professionalism) is 0. With the step a: consistency and versatility move by random(2a)
/// / 10, leadership by random(2a) / (random(6) + 10), held to 1..20.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4f63..0x5d50ab.
pub(crate) unsafe fn development_mild_attributes(d: &mut Development) {
    use exe::utils_random as random;
    let pd = PlayerSheet(d.pd);
    let stamina = || pd.stamina() as i32;
    let fitness = || pd.natural_fitness() as i32;
    if d.mark < 0 {
        if fitness() > stamina() + 2 || random(fitness()) > random(20) {
            d.mark = 0;
        } else {
            d.mark = d.mark.wrapping_add(random(d.mark as i32 + 1) as i8);
            if d.mark > 0 && stamina() > 15 {
                d.mark = random(d.mark as i32) as i8;
            }
        }
    } else if d.mark > 0 && stamina() > 15 {
        d.mark = random(d.mark as i32) as i8;
    }
    sd_clamp_attribute(pd.addr(), player_sheet::STAMINA, d.mark as i32, 1, 20);
    if stamina() > fitness() + 5 && random(read_i8(d.p + staff::PROFESSIONALISM) as i32) == 0 {
        pd.set_stamina(((pd.stamina() as u8).wrapping_sub(random(2) as u8)) as i8);
    }
    let two = (d.step as i32) * 2;
    sd_clamp_attribute(pd.addr(), player_sheet::CONSISTENCY, random(two) / 10, 1, 20);
    let r = random(two);
    let lead = r / (random(6) + 10);
    sd_clamp_attribute(pd.addr(), player_sheet::LEADERSHIP, lead, 1, 20);
    sd_clamp_attribute(pd.addr(), player_sheet::VERSATILITY, random(two) / 10, 1, 20);
}

/// Anticipation and decisions of a developing player (0x5d50ab..0x5d5115): each one moves by random(saved growth),
/// held to -125..125, when random(saved growth) beats random(100).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d50ab..0x5d5115.
pub(crate) unsafe fn development_reading_of_the_game(d: &Development) {
    use exe::utils_random as random;
    for field in [player_sheet::ANTICIPATION, player_sheet::DECISIONS] {
        if random(d.saved_growth) > random(100) {
            sd_clamp_attribute(d.pd, field, random(d.saved_growth), -0x7d, 0x7d);
        }
    }
}

/// A 30-year-old's or older player's ageing (0x5d5115..0x5d5771), by the age byte.
/// Past 37: a forward rated 20 (attacking midfielder or attacker) over 50 ability loses random(20) when random(ability)
/// beats professionalism + stamina + 10, a defender, midfielder, sweeper or defensive midfielder rated 20 random(10)
/// against professionalism + stamina; jumping loses random(3) above 15, else one in three above 5 random(2);
/// acceleration and pace lose 1 + random(3) above 8 and then one in three above 4; agility one in three above 10,
/// natural fitness above 4; stamina and strength random(3) above 7, else one in three above 4.
/// 35 to 37: forwards rated 20 lose random(10) like the over-37 ones; jumping random(2) above 17, else one in three
/// random(2) above 10; acceleration random(3) above 15, else one in three above 5; pace random(3) above 15 and then one
/// in three above 5; agility one in three above 10, natural fitness above 8; stamina and strength random(3) above 15,
/// else one in three above 8 (strength above 10).
/// 33 and 34: a defender or defensive midfielder rated 20, or any goalkeeper rating, with professionalism above
/// random(11) + 10, work rate above random(15) + 6 and random(professionalism) not 0, gains random(2) of anticipation
/// and of positioning; forwards rated 20 over 100 ability lose random(6) when random(ability) beats professionalism +
/// stamina + 50; acceleration and pace lose random(2) above 15, else one in three above 10; agility one in three at
/// 20, natural fitness above 15, stamina above 18; strength at 20 loses one unless random(3) is 0 (two times in three,
/// against one in three everywhere else: [inference] a defect of the original).
/// 30 to 32: a defender, defensive midfielder, goalkeeper or sweeper rated 20 with professionalism above random(11) +
/// 10, work rate above random(10) + 6 and random(professionalism) not 0 gains random(2) of anticipation and of
/// positioning while they are under 15; an attacker, attacking midfielder or midfielder rated 20 with professionalism
/// above random(6) + 15, work rate above random(11) + 10 and random(professionalism) not 0 gains random(2) of
/// anticipation under 15; acceleration and pace lose one in three above 18.
/// The gains are `clamp_char_sum` -125..125.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d5115..0x5d5771.
pub(crate) unsafe fn development_ageing(p: usize, pd: usize) {
    let pd = PlayerSheet(pd);
    let p = Staff(p);
    use exe::utils_random as random;
    let age = p.age() as i8;
    let rated_20 = |field: usize| read_u8(pd.addr() + field) == 20;
    let forward = || rated_20(player_sheet::ATTACKING_MIDFIELDER) || rated_20(player_sheet::ATTACKER);
    let gain = |field: usize| {
        let r = random(2);
        if r != 0 {
            sd_clamp_attribute(pd.addr(), field, r, -0x7d, 0x7d);
        }
    };
    if age < 30 {
        return;
    }
    if age > 37 {
        covered!("gra_2c ageing past 37");
        if forward() {
            sd_veteran_ability_loss(p.addr(), pd.addr(), 50, 10, 20);
        } else if rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::MIDFIELDER) || rated_20(player_sheet::SWEEPER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) {
            sd_veteran_ability_loss(p.addr(), pd.addr(), 50, 0, 10);
        }
        if pd.jumping() > 15 {
            pd.set_jumping(((pd.jumping() as u8).wrapping_sub(random(3) as u8)) as i8);
        } else if pd.jumping() > 5 && random(3) == 0 {
            pd.set_jumping(((pd.jumping() as u8).wrapping_sub(random(2) as u8)) as i8);
        }
        for field in [player_sheet::ACCELERATION, player_sheet::PLAYER_PACE] {
            if read_i8(pd.addr() + field) > 8 {
                write_u8(pd.addr() + field, read_u8(pd.addr() + field).wrapping_add((-1 - random(3)) as u8));
            }
            sd_maybe_lose_one(pd.addr(), field, 4);
        }
        sd_maybe_lose_one(pd.addr(), player_sheet::AGILITY, 10);
        sd_maybe_lose_one(pd.addr(), player_sheet::NATURAL_FITNESS, 4);
        sd_lose_some(pd.addr(), player_sheet::STAMINA, 7, 3, 4);
        sd_lose_some(pd.addr(), player_sheet::STRENGTH, 7, 3, 4);
    } else if age > 34 {
        if forward() {
            sd_veteran_ability_loss(p.addr(), pd.addr(), 50, 10, 10);
        }
        let jumping = pd.jumping();
        if jumping > 17 || (jumping > 10 && random(3) == 0) {
            pd.set_jumping(((pd.jumping() as u8).wrapping_sub(random(2) as u8)) as i8);
        }
        sd_lose_some(pd.addr(), player_sheet::ACCELERATION, 15, 3, 5);
        if pd.player_pace() > 15 {
            pd.set_player_pace(((pd.player_pace() as u8).wrapping_sub(random(3) as u8)) as i8);
        }
        sd_maybe_lose_one(pd.addr(), player_sheet::PLAYER_PACE, 5);
        sd_maybe_lose_one(pd.addr(), player_sheet::AGILITY, 10);
        sd_maybe_lose_one(pd.addr(), player_sheet::NATURAL_FITNESS, 8);
        sd_lose_some(pd.addr(), player_sheet::STAMINA, 15, 3, 8);
        sd_lose_some(pd.addr(), player_sheet::STRENGTH, 15, 3, 10);
    } else if age > 32 {
        if (rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) || (pd.goalkeeper() as u8) != 0)
            && p.professionalism() as i32 > random(11) + 10
            && pd.work_rate() as i32 > random(15) + 6
            && random(p.professionalism() as i32) != 0
        {
            gain(player_sheet::ANTICIPATION);
            gain(player_sheet::POSITIONING);
        }
        if forward() {
            sd_veteran_ability_loss(p.addr(), pd.addr(), 100, 50, 6);
        }
        sd_lose_some(pd.addr(), player_sheet::ACCELERATION, 15, 2, 10);
        sd_lose_some(pd.addr(), player_sheet::PLAYER_PACE, 15, 2, 10);
        if rated_20(player_sheet::AGILITY) && random(3) == 0 {
            pd.set_agility(((pd.agility() as u8).wrapping_sub(1)) as i8);
        }
        sd_maybe_lose_one(pd.addr(), player_sheet::NATURAL_FITNESS, 15);
        sd_maybe_lose_one(pd.addr(), player_sheet::STAMINA, 18);
        // defect of the original [inference]: strength at 20 loses one unless random(3) is 0
        if rated_20(player_sheet::STRENGTH) && random(3) != 0 {
            pd.set_strength(((pd.strength() as u8).wrapping_sub(1)) as i8);
        }
    } else {
        covered!("gra_2c ageing 30 to 32");
        let professionalism = || p.professionalism() as i32;
        let work_rate = || pd.work_rate() as i32;
        if rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) || rated_20(player_sheet::GOALKEEPER) || rated_20(player_sheet::SWEEPER) {
            if professionalism() > random(11) + 10 && work_rate() > random(10) + 6 && random(professionalism()) != 0 {
                if pd.anticipation() < 15 {
                    gain(player_sheet::ANTICIPATION);
                }
                if pd.positioning() < 15 {
                    gain(player_sheet::POSITIONING);
                }
            }
        } else if (rated_20(player_sheet::ATTACKER) || rated_20(player_sheet::ATTACKING_MIDFIELDER) || rated_20(player_sheet::MIDFIELDER))
            && professionalism() > random(6) + 15
            && work_rate() > random(11) + 10
            && random(professionalism()) != 0
            && pd.anticipation() < 15
        {
            gain(player_sheet::ANTICIPATION);
        }
        sd_maybe_lose_one(pd.addr(), player_sheet::ACCELERATION, 18);
        sd_maybe_lose_one(pd.addr(), player_sheet::PLAYER_PACE, 18);
    }
}

pub(crate) unsafe fn sd_pick_favoured_team_mate(p: usize, club: usize) -> usize {
    let f: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_PICK_FAVOURED_TEAM_MATE);
    f(p, club)
}

pub(crate) unsafe fn sd_remove_dispute(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISPUTE_REMOVE_DISPUTE);
    f(read_usize(G_DISPUTE), p)
}

pub(crate) unsafe fn sd_club_reputation_for_staff(club: usize, p: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_REPUTATION_FOR_STAFF);
    f(club, p)
}

/// The person's state record byte +0xa: the day's unrest counter [inference: named injury counter in the fields].
pub(crate) unsafe fn sd_unrest(d: &StaffDay) -> usize {
    read_usize(G_STAFF_STATE_TABLE) + d.state_off + staff_state::INJURY_COUNTER
}

/// Adds `n` to the unrest counter (8 bits, wrapping).
pub(crate) unsafe fn sd_add_unrest(d: &StaffDay, n: i32) {
    let a = sd_unrest(d);
    write_u8(a, read_u8(a).wrapping_add(n as u8));
}

/// One grievance's roll: random(morale + 200) below random(`odds`) adds random(5) to the unrest.
pub(crate) unsafe fn sd_grievance_roll(d: &StaffDay, odds: i32) {
    let a = exe::utils_random(read_i8(d.contract + contract::MORALE) as i32 + 200);
    if a < exe::utils_random(odds) {
        sd_add_unrest(d, exe::utils_random(5));
    }
}

/// The club's state record (`G_CLUB_STATE_TABLE`, 6 bytes a club): byte `k`, signed.
pub(crate) unsafe fn sd_club_state(club: usize, k: usize) -> i8 {
    read_i8(read_usize(G_CLUB_STATE_TABLE) + read_u32(club) as usize * 6 + k)
}

/// The grievances word of the contract.
pub(crate) unsafe fn sd_grievances(d: &StaffDay) -> u32 {
    read_u32(d.contract + contract::GRIEVANCES)
}

/// The game's average match rating: total / count of `G_PLAYER_STATS` (+0x10 / +0x18), 6.8 while the count is 0.
pub(crate) unsafe fn sd_average_rating() -> f64 {
    let stats = read_usize(G_PLAYER_STATS);
    let count = read_f64(stats + 0x18);
    // fcomp + test 0x40: equal (or NaN) takes 6.8
    if count == 0.0 || count.is_nan() {
        read_f64(rdata::F64_6_8)
    } else {
        read_f64(stats + 0x10) / count
    }
}

/// A player stat of the season (`PlayerStats::get_stat`, default 6.5) as the original stores it, a float.
pub(crate) unsafe fn sd_stat(p: usize, category: u32, which: u32) -> f32 {
    let p = Staff(p);
    exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), p.player().addr(), category, which, 6.5) as f32
}

/// Flag 4 of the contract (0x5cf969..0x5cfa3a): cleared without a club, below state +5 of 3, or when random(pressure)
/// beats random(state + 1); set one time in (pressure step + state +5 + 1) for an ambitious and professional (both above
/// 10) player at a club whose state +5 is above step + 7.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf969..0x5cfa3a.
pub(crate) unsafe fn staff_day_european_flag(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = d.club;
    let contract = d.contract;
    if exe::staff_contract_get_flags(contract) & 4 != 0 {
        if club == 0 {
            exe::staff_contract_clear_flags(contract, 4);
            return;
        }
        let state = sd_club_state(club, 5);
        if state < 3 {
            exe::staff_contract_clear_flags(contract, 4);
            return;
        }
        let a = random(state as i32 + 1);
        if random(p.pressure() as i32) > a {
            exe::staff_contract_clear_flags(contract, 4);
        }
        return;
    }
    if club == 0 {
        return;
    }
    let pressure = p.pressure();
    let step: i8 = if pressure == 20 {
        5
    } else if pressure > 15 {
        3
    } else if pressure > 10 {
        2
    } else {
        (pressure > 5) as i8
    };
    if p.ambition() > 10
        && p.professionalism() > 10
        && sd_club_state(club, 5) as i32 > step as i32 + 7
        && random(pressure as i32 + step as i32 + 1) == 0
    {
        exe::staff_contract_set_flags(contract, 4);
    }
}

/// The player's last match rating (`PlayerStats::get_last_match_rating`).
pub(crate) unsafe fn sd_last_rating(p: usize) -> f32 {
    let p = Staff(p);
    let f: unsafe extern "thiscall" fn(usize, usize) -> f32 = core::mem::transmute(FN_PLAYERSTATS_GET_LAST_MATCH_RATING);
    f(read_usize(G_PLAYER_STATS), p.player().addr())
}

/// A media story praising the player (`Media::post_player_praise_story`, no fixture record).
pub(crate) unsafe fn sd_praise_story(p: usize, club: usize, topic: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) = core::mem::transmute(FN_MEDIA_POST_PLAYER_PRAISE_STORY);
    f(read_usize(G_MEDIA), p, club, topic, 0);
}

/// The 8-byte date `G_CURRENT_DATE` + `days` (`Date::add_days`), as its day and year.
pub(crate) unsafe fn sd_date_in(days: u32) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, days);
    (read_i16(r), read_u16(r + 2))
}

/// The first scheduled date of a competition (`Comp::get_first_schedule_date`), as its day and year.
pub(crate) unsafe fn sd_first_date(comp: usize) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::comp_get_first_schedule_date(comp, out.as_mut_ptr() as usize);
    (read_i16(r), read_u16(r + 2))
}

/// The criticism half of the club stories (0x5d08bb..0x5d0b42): a player whose ratings trail tells the media; with a
/// future transfer he goes (kind 0xc) when the average less random(5) / 10 beats his form; a recent signing (under 180
/// days less pressure) mostly (9 in 10) complains of the club (kind 2); with form and other rating apart from the
/// average by 0.3, an unsettled one (random(unrest + 20) above random(50)) says more (kind 0xd); else kind 0.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d08bb..0x5d0b42.
pub(crate) unsafe fn staff_day_club_criticism(d: &StaffDay, r: &ClubStoryRatings) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let avg = r.average as f64;
    let season = r.season as f64;
    let other = r.other as f64;
    let form = r.form as f64;
    let speaks = 'speaks: {
        let n = random(5);
        if n as f64 * read_f64(rdata::F64_0_1) + (avg - read_f64(rdata::F64_0_3)) > other && !((random(10) + 5) as f64 >= season) {
            break 'speaks true;
        }
        if avg + read_f64(rdata::F64_0_1) > other
            && avg - read_f64(rdata::F64_0_7) > form
            && !((random(10) + 5) as f64 >= season)
        {
            break 'speaks true;
        }
        let low = if !(form >= other) { form } else { other };
        let gap = (read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 - club.reputation() as i32) as f64;
        gap * read_f64(rdata::F64_0_0001428571429) + avg + read_f64(rdata::F64_0_1) > low // 1/7000
            && avg + read_f64(rdata::F64_0_35) > other
            && !((random(15) + 5) as f64 >= season)
            && !(r.last as f64 > read_f64(rdata::F64_6))
    };
    if !speaks {
        return;
    }
    let story = |kind: u32| exe::media_create_player_media_story(read_usize(G_MEDIA), p, club, kind, 0, 0);
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        let n = random(5);
        if avg - n as f64 * read_f64(rdata::F64_0_1) > form {
            covered!("gra_2b leaving player criticises the club");
            story(0xc);
            return;
        }
    }
    let days = exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, read_u32(p.addr() + staff::DATE_JOINED_CLUB + 4));
    if days < 180 - p.pressure() as i32 && random(10) != 0 {
        covered!("gra_2b new signing criticises the club");
        story(2);
        return;
    }
    if avg + read_f64(rdata::F64_0_3) > form && other > form + read_f64(rdata::F64_0_3) {
        let a = random(read_u8(sd_unrest(d)) as i32 + 20);
        if a > random(50) {
            covered!("gra_2b unsettled player criticises the club");
            story(0xd);
            return;
        }
    }
    covered!("gra_2b player criticises the club");
    story(0);
}

/// The praise half of the club stories (0x5d0b42..0x5d0e27): a player whose form beats the average + 0.35 less the
/// standing bonus, with a last rating of 7 or more. A season rating above 10 one time in 10 with the other rating or the
/// form well above the average thanks the manager when the manager favours him (one time in (25 - loyalty) × 4);
/// otherwise a chain of draws against the ratings may praise him (topic 0x2b).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0b42..0x5d0e27.
pub(crate) unsafe fn staff_day_club_praise(d: &StaffDay, r: &ClubStoryRatings, bonus: f64) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let avg = r.average as f64;
    let season = r.season as f64;
    let other = r.other as f64;
    let form = r.form as f64;
    if avg + read_f64(rdata::F64_0_35) - bonus >= form || !(r.last as f64 >= read_f64(rdata::F64_7)) {
        return;
    }
    if r.season > read_f32(rdata::F32_10) && random(10) == 0 {
        if !(avg + read_f64(rdata::F64_0_4) >= other) || !(avg + read_f64(rdata::F64_1_1) >= form) {
            if exe::staff_is_favoured_by(club.manager().addr(), p) != 0
                && random((25 - p.loyalty() as i32) * 4) == 0
            {
                covered!("gra_2b player thanks the manager");
                let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_NEWS_POST_PLAYER_THANKS_MANAGER);
                f(p.addr(), club.addr());
            }
            return;
        }
    }
    let praise = 'praise: {
        if !((random(5) + 3) as f64 >= season) && !(avg + read_f64(rdata::F64_1_6) >= other) {
            break 'praise true;
        }
        let ratio = r.ratio as f64;
        let n = random(4);
        if !(n as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_0_55) >= ratio)
            && !(avg + read_f64(rdata::F64_0_1) >= form)
            && !((random(20) + 4) as f64 >= season)
        {
            break 'praise true;
        }
        if !((random(20) + 10) as f64 >= season) {
            let n = random(10);
            if !(n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_8) >= other) {
                break 'praise true;
            }
        }
        if !((random(20) + 10) as f64 >= season) {
            let both = (form + r.other as f64) * read_f64(rdata::F64_0_5);
            let n = random(10);
            if !(n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_8) >= both) {
                let a = random(club.reputation() as i32 * 2);
                if random(read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) as i32) > a {
                    break 'praise true;
                }
            }
        }
        if (random(20) + 10) as f64 >= season {
            break 'praise false;
        }
        let n = random(16);
        if n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_3) >= other {
            break 'praise false;
        }
        let k = random(2);
        let state = read_usize(G_STAFF_STATE_TABLE) + read_u32(p.addr()) as usize * staff_state::SIZE;
        (read_u8(state + staff_state::STATUS_FLAGS) & 0xf) as i32 > k
    };
    if praise {
        covered!("gra_2b player praised by the club's media");
        sd_praise_story(p.addr(), club.addr(), 0x2b);
    }
}

/// Days from the date at `from` (8 bytes) to today, counted as the original's inline (365 a year, no leap days).
pub(crate) unsafe fn sd_days_since_inline(from: usize) -> i32 {
    (read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(from + 2) as i32) * 365 - read_i16(from) as i32 + read_i16(G_CURRENT_DATE) as i32
}

/// `Date::days_since` from a stored date (two dwords at `at`) to today.
pub(crate) unsafe fn sd_days_since(at: usize) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, read_u32(at), read_u32(at + 4))
}

/// One time in ten the player dislikes the club, else its manager (`staff_add_disliked_club` / `_staff`, forced).
pub(crate) unsafe fn sd_dislike_club_or_manager(d: &StaffDay) {
    if exe::utils_random(10) == 0 {
        covered!("gra_2b player dislikes the club");
        exe::staff_add_disliked_club(d.p, d.club, 1);
    } else {
        covered!("gra_2b player dislikes the manager");
        exe::staff_add_disliked_staff(d.p, read_usize(d.club + club::MANAGER), 1);
    }
}

/// The days at the club counted for a new contract or a move (0x5d1495..0x5d1530): days since the contract started
/// (at most 650) plus, for a capped first team international at a club under reputation 5500, (caps + 20) × 10.
pub(crate) unsafe fn sd_contract_days(p: usize, club: usize, mut days: i32) -> i32 {
    let p = Staff(p);
    let club = Club(club);
    if p.national_job().addr() != 0
        && p.int_apps() != 0
        && p.job_for_nation() == 11
        && club.reputation() < 0x157c
    {
        days = days.wrapping_add((p.int_apps() as i32 + 20) * 10);
    }
    days
}

/// The player's pull: reputation + 2 × (days - 25 × (morale + 2 × loyalty + 22)).
pub(crate) unsafe fn sd_pull(d: &StaffDay, days: i32) -> i32 {
    let c = read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 + read_i8(d.p + staff::LOYALTY) as i32 * 2 + 22;
    read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 + 2 * (days - 25 * c)
}

/// `date` (day, year) falls before today.
pub(crate) unsafe fn sd_before_today((day, year): (i16, u16)) -> bool {
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    year < today_year || (year == today_year && day < read_i16(G_CURRENT_DATE))
}

/// The date at `this` (8 bytes) + `days` (`Date::add_days`), as its day and year.
pub(crate) unsafe fn sd_add_days(this: usize, days: i32) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::date_add_days(this, out.as_mut_ptr() as usize, days as u32);
    (read_i16(r), read_u16(r + 2))
}

/// The satisfaction talk (0x5d188f..0x5d1eef), see `staff_day_contract_talks`.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d188f..0x5d1eef.
pub(crate) unsafe fn staff_day_new_contract(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = d.club;
    let contract = Contract(d.contract);
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    let entry = exe::shortlist_manager_find_club_shortlist_entry(shortlists, club, p, 0, 0);
    let satisfaction = sd_contract_satisfaction(contract.addr(), p.addr()) as i8;
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let loyalty = p.loyalty();
    // the entry's status (bits 1..4 of +0x24) and whether the entry is of this player (+0xf)
    let status = |entry: usize| (read_u8(entry + 0x24) >> 1) & 0xf;
    let own = |entry: usize| read_u32(entry + 0xf) == read_u32(p.addr());
    let ask_publicly = |temper: i8| {
        let pd = PlayerSheet(d.pd);
        if pd.current_reputation() > 0x1482
            && p.ambition() > 5
            && p.temperament() < temper
            && p.sportsmanship() < 15
            && p.professionalism() < 10
            && random(14) == 0
        {
            sd_dislike_club_or_manager(d);
        }
    };
    match satisfaction {
        3 => {
            if entry == 0 {
                return;
            }
            if g(contract.addr()) & 2 != 0 {
                // one day in 8, with grievance 2 alone and no offer pending, he says so
                let mark = (read_u32(G_CURRENT_DATE).wrapping_add(read_u32(p.addr())) & 7) != 0;
                if mark || g(contract.addr()) != 2 || exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p.addr()), contract.club() as u32) != 0 {
                    return;
                }
                covered!("gra_2b player wants a new contract publicly");
                sd_post_wants_new_contract_news(contract.addr());
                ask_publicly(10);
                return;
            }
            if sd_days_since(contract.addr() + contract::DATE_STARTED) < (loyalty as i32 + 25) * 10
                || contract.morale() >= -25
                || sd_days_to_expiry(contract.addr()) >= 565
            {
                return;
            }
            let ambition = p.ambition();
            if ambition <= 10 || loyalty >= 10 || contract.happiness() <= 75 {
                return;
            }
            let pull = ((ambition as i16).wrapping_sub((loyalty as i16).wrapping_mul(2)))
                .wrapping_mul(25)
                .wrapping_add((p.pressure() as i16).wrapping_mul(10))
                .wrapping_add(read_i16(d.pd + player_sheet::WORLD_REPUTATION));
            if pull <= 0x1964 {
                return;
            }
            if exe::human_manager_is_human_club(sd_contract_club(contract.addr()), 0, 1, 0, 0) == 0 {
                let s = status(entry);
                let offered = match s {
                    1 => sd_offer_new_contract(p.addr(), own(entry)),
                    2 | 3 => sd_offer_new_contract(p.addr(), false),
                    _ => return,
                };
                if offered != 0 {
                    return;
                }
            }
            sd_ask_for_contract(d);
        }
        2 => {
            if g(contract.addr()) & 2 != 0 {
                staff_day_add_morale(contract.addr(), 10);
                exe::staff_contract_clear_grievances(contract, 2);
            }
        }
        1 => {
            if entry == 0 {
                return;
            }
            if g(contract.addr()) & 2 != 0 {
                staff_day_contract_reminder(d, entry);
                return;
            }
            if sd_days_since(contract.addr() + contract::DATE_STARTED) < (loyalty as i32 + 25) * 10 {
                return;
            }
            if sd_days_to_expiry(contract.addr()) > 180
                && exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p, contract) == 7
            {
                staff_day_add_morale(contract.addr(), -5);
                return;
            }
            let age = p.age() as i8;
            let veteran = if read_i8(p.player().addr() + player_sheet::GOALKEEPER) == 20 { age > 34 } else { age > 31 };
            if veteran {
                let floor = (p.professionalism() as i8).wrapping_add(loyalty).wrapping_neg();
                if contract.morale() > floor && sd_days_to_expiry(contract.addr()) > (loyalty as i32 + 30) * 5 {
                    return;
                }
            }
            if age < 21
                && contract.happiness() < (loyalty as u8).wrapping_shl(1).wrapping_add(15) as i8
                && sd_days_to_expiry(contract.addr()) > (25 - loyalty as i32) * 10
            {
                return;
            }
            if exe::human_manager_is_human_club(sd_contract_club(contract.addr()), 0, 1, 0, 0) != 0 {
                if g(contract.addr()) == 0 {
                    covered!("gra_2b player asks a human club for a contract");
                    exe::staff_contract_apply_event_flags(contract, 2, 0);
                } else {
                    staff_day_add_morale(contract.addr(), -5);
                }
                return;
            }
            let s = status(entry);
            let offered = match s {
                1 => sd_offer_new_contract(p.addr(), own(entry)),
                2..=6 => sd_offer_new_contract(p.addr(), false),
                _ => return,
            };
            if offered == 0 {
                sd_ask_for_contract(d);
            }
        }
        _ => {}
    }
}

/// A loan player (contract type 1) without a loan contract, offers or a free transfer pending, whose contract ended
/// more days ago than random(100) (0x5d1eef..0x5d1fd4): wants a new contract (grievance 2) and loses that many days of
/// morale (8 bits). Returns false when the loan conditions do not hold.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1eef..0x5d1fd4.
pub(crate) unsafe fn staff_day_contract_running_out(d: &StaffDay) -> bool {
    let p = d.p;
    let contract = Contract(d.contract);
    if contract.contract_type() & 0x3f != 1
        || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0
        || exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0, 0) != 0
        || exe::transfer_manager_get_exchange_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
        || exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0
    {
        return false;
    }
    let days = sd_days_since_inline(contract.addr() + contract::CONTRACT_EXPIRES);
    if days <= exe::utils_random(100) {
        return true;
    }
    covered!("gra_2b loan player wants a contract");
    if contract.grievances() & 2 == 0 {
        exe::staff_contract_apply_event_flags(contract, 2, 0);
    }
    staff_day_add_morale(contract.addr(), (days as u8).wrapping_neg() as i8 as i32);
    true
}

/// A veteran far from home wants to go back (0x5d1fd4..0x5d24e7): without grievance 0x100, loyalty under 17, past 25
/// (past 31 only abroad), not queued for a free transfer, with a world reputation above 4250 and reputations above
/// random draws over the club's, one day in 50: the days at the club (or in the contract, under 250 + random(150)),
/// raised for a foreigner near his contract's end (under 300 days) and lowered past 600 days, against a threshold of
/// loyalty, reputation and two seeded squares (`G_GAME_RANDOM_SEED`), raised when he favours the club, by the club's
/// standing and its European place [inference: club state +0 or club +0x1db set], lowered for a foreigner: above it
/// (and above 700) he loses 20 morale and gets grievance 0x100. Returns false when the conditions fail (the relations
/// follow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1fd4..0x5d238e.
pub(crate) unsafe fn staff_day_going_home(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if contract.grievances() & 0x100 != 0 || p.loyalty() >= 17 {
        return false;
    }
    let age = p.age() as i8;
    if age <= 25 || (age >= 32 && (club.nation().addr() as u32) == (p.nation().addr() as u32)) {
        return false;
    }
    if exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0 {
        return false;
    }
    let pd = p.player();
    if pd.world_reputation() <= 0x109a {
        return false;
    }
    let current = pd.current_reputation() as f64;
    let r = random(2000);
    if r as f64 + club.reputation() as f64 * read_f64(rdata::F64_0_5) >= current {
        return false;
    }
    let world = read_i16(p.player().addr() + player_sheet::WORLD_REPUTATION) as f64;
    let r = random(1000);
    if r as f64 + club.reputation() as f64 * read_f64(rdata::F64_0_25) >= world {
        return false;
    }
    if random(50) != 0 {
        return false;
    }
    let mut days = sd_days_since_inline(p.addr() + staff::DATE_JOINED_CLUB);
    let in_contract = sd_days_since_inline(contract.addr() + contract::DATE_STARTED);
    if in_contract < random(150) + 250 {
        days = in_contract;
    }
    let left = -sd_days_since_inline(contract.addr() + contract::CONTRACT_EXPIRES);
    let foreign = exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0;
    if foreign && left < 300 {
        days = days + left + 400;
    } else if left > 600 {
        days += (600 - left) * 10;
    }
    let id = read_i32(p.addr());
    let seeded = id.wrapping_add(read_i32(G_GAME_RANDOM_SEED));
    let a = game_pow((seeded % 31) as f64, read_f64(rdata::F64_2));
    let b = game_pow((seeded % 26) as f64, read_f64(rdata::F64_2));
    let loyalty = p.loyalty() as i32;
    let base = (150 * loyalty + 0x898) as f64 - read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as f64 * read_f64(rdata::F64_0_1333333333);
    let mut bar = ftol(base + a - b);
    if exe::staff_favours_club(p, club) != 0 {
        bar = ftol(((id % 6) as f64 * read_f64(rdata::F64_0_5555555556) + read_f64(rdata::F64_1_25)) * bar as f64);
        if left > 300 {
            bar = ftol(((id % 7) as f64 * read_f64(rdata::F64_0_5555555556) + read_f64(rdata::F64_1_25)) * bar as f64);
        }
    }
    let state = read_usize(G_CLUB_STATE_TABLE) + read_u32(club.addr()) as usize * 6;
    let standing = read_i8(state + 4);
    if standing >= 0 {
        bar += (loyalty + 30) * (standing as i32 + 1);
    }
    if read_i32(state) != -1 || (club.continental_comp_id() as i32) != -1 {
        bar += (loyalty + 10) * 50;
    }
    if exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0 {
        let k = (id.wrapping_add(read_i32(club.addr())) % 11) as f64;
        bar = ftol((read_f64(rdata::F64_0_75) - k * read_f64(rdata::F64_0_05)) * bar as f64);
    }
    if days > 700 && days >= bar {
        covered!("gra_2b veteran wants to go home");
        staff_day_add_morale(contract.addr(), -20);
        exe::staff_contract_apply_event_flags(contract, 0x100, 0);
    }
    true
}

/// The dressing room (0x5d24e7..0x5d28f8): one day in 50 × professionalism², his morale moves by the club he favours
/// (+random(100)) or dislikes (random(-100), grievance 0x40 under -50), by the manager who favours (+random(75)) or
/// dislikes him (random(-75), grievance 8 under -50), by the disliked staff at the club (random(-25 × count)) and the
/// favoured ones (random(25 × count)); his three disliked team mates [inference: preferences +0x28/+0x2c/+0x30] who
/// are listed at the club lift him by random(loyalty + 10). Without a reason left he drops grievances 0x40 and 8.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d24e7..0x5d28f8.
pub(crate) unsafe fn staff_day_relations(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    if club.is_null() || random(50) != 0 {
        return;
    }
    let prof = p.professionalism() as i32;
    if random(prof * prof) != 0 {
        return;
    }
    covered!("gra_2b dressing room relations");
    let add = |n: i32| staff_day_add_morale(contract.addr(), n as i8 as i32);
    let unhappy = |flag: u32| {
        if contract.morale() < -50 {
            exe::staff_contract_apply_event_flags(contract, flag, 0);
        }
    };
    let mut reason = false;
    if exe::staff_favours_club(p, club) != 0 {
        add(random(100));
    } else if exe::staff_dislikes_club(p, club) != 0 {
        add(random(-100));
        unhappy(0x40);
        reason = true;
    }
    if exe::staff_is_favoured_by(club.manager().addr(), p) != 0 {
        add(random(75));
    } else if exe::staff_is_disliked_by(club.manager().addr(), p) != 0 {
        add(random(-75));
        unhappy(8);
        reason = true;
    }
    let disliked = sd_count_disliked_at_club(p.addr(), club.addr()) as i8 as i32;
    if disliked != 0 {
        add(random(-25 * disliked));
        reason = true;
        let prefs = p.staff_preferences();
        if prefs != 0 {
            for slot in [0x28usize, 0x2c, 0x30] {
                let other = read_usize(prefs + slot);
                if other == 0 {
                    continue;
                }
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), other);
                write_usize(G_CONTRACT_LOOKUP_RESULT, c);
                let at = if c != 0 { sd_contract_club(c) } else { 0 };
                if at == club.addr() && exe::staff_is_transfer_listed_or_requested(read_usize(p.staff_preferences() + slot), club) != 0 {
                    add(random(p.loyalty() as i32 + 10));
                    if contract.morale() > 50 {
                        reason = false;
                    }
                }
            }
        }
        unhappy(0x40);
    }
    let favoured = sd_count_favoured_at_club(p.addr(), club.addr()) as i8 as i32;
    if favoured != 0 {
        add(random(25 * favoured));
    }
    if !reason {
        exe::staff_contract_clear_grievances(contract, 0x40);
        exe::staff_contract_clear_grievances(contract, 8);
    }
}

/// The rest of a contracted player's day (0x5d2392..0x5d352d): grievances 0x10 and 4 end once in the side (left out
/// at most once, without a linked B team); a dropped, unsettled, famous one may tell the media he wants first team
/// football (`Media::queue_wants_first_team_story`, contract flag 0x200); grievance 0x20000 (homesick [inference]) ends
/// in his home region, else may go to the media (kind 1); contract flag 1 and grievance 0x800 (a disliked team mate)
/// end when no disliked person shares his club, else cost morale; an ambitious first team player whose deserved
/// status is 1 to 3 and who has a transfer pending may talk (kind 3); grievances 0x200, 0x400, 0x80, 0x10000, 0x800000,
/// 0x4000 and 0x8000 fade; he learns the club's language (squad status nibble, by adaptability) or, abroad, grows
/// homesick by the days at the club against his character, losing morale (grievance 0x20000), and at last, with
/// grievance 0x20, may go absent (`AWOL::start_absence`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d2392..0x5d352d.
pub(crate) unsafe fn staff_day_settling(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = Club(d.club);
    let contract = Contract(d.contract);
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let left_out = |c: usize| read_i8(c + contract::MATCHES_LEFT_OUT);
    // the ambition and temperament tests of the original all give way to "left out more than once"
    for bit in [0x10u32, 4] {
        if g(contract.addr()) & bit != 0 && d.b_team == 0 && left_out(contract.addr()) > 1 {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    let pressure = read_i8(p + staff::PRESSURE) as i32;
    let prof = read_i8(p + staff::PROFESSIONALISM) as i32;
    let loyalty = read_i8(p + staff::LOYALTY) as i32;
    if g(contract.addr()) & 0x10 != 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
        && contract.morale() < -25
        && left_out(contract.addr()) < 2
        && pressure < 6
        && random(prof + 5) == 0
        && read_usize(p + staff::PLAYER) != 0
        && read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x109a
        && (read_i16(G_CURRENT_DATE) as i32 % 7).wrapping_add(read_i32(p)) == 0
    {
        covered!("gra_2b dropped player wants first team football");
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MEDIA_QUEUE_WANTS_FIRST_TEAM_STORY);
        f(read_usize(G_MEDIA), p, 0);
        exe::staff_contract_set_flags(contract, 0x200);
    }
    if g(contract.addr()) & 0x20000 != 0 {
        let home = Nation(read_usize(p + staff::NATION));
        if home.region() == read_u8(club.nation().addr() + nation::REGION) {
            exe::staff_contract_clear_grievances(contract, 0x20000);
        } else if read_u8(p + staff::JOB_FOR_CLUB) == 11
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
            && !d.away_from_job
            && read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x109a
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 12 == 4
            && prof < random(5) + 5
            && loyalty < random(5) + 5
        {
            covered!("gra_2b homesick player talks");
            sd_media_outburst(d, 1, 5);
        }
    }
    if exe::staff_contract_get_flags(contract) & 1 != 0 {
        let temper = (read_i8(p + staff::TEMPERAMENT) as u8).wrapping_sub(read_i8(p + staff::PROFESSIONALISM) as u8).wrapping_add(25) as i8;
        let (found, _) = sd_disliked_team_mate(p);
        if !found || random(temper as i32) == 0 {
            exe::staff_contract_clear_flags(contract, 1);
        } else {
            staff_day_add_morale(contract.addr(), -5);
        }
    }
    if g(contract.addr()) & 0x800 != 0 {
        let (found, _) = sd_disliked_team_mate(p);
        if found {
            staff_day_add_morale(contract.addr(), -5);
        } else {
            exe::staff_contract_clear_grievances(contract, 0x800);
        }
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 11
        && read_i8(p + staff::AGE) > 23
        && !d.away_from_job
        && club.division().addr() != 0
        && contract.unhappy_flag() > 2
    {
        let ambition = read_i8(p + staff::AMBITION) as i32;
        let r = random(((65 - ambition) * 2 + read_i8(p + staff::TEMPERAMENT) as i32) * 25);
        let status = |s: u8| exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p, contract) == s;
        if read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 > r + 4000
            && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
            && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, club.division().addr(), 1, 0) == 0
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
            && (status(1) || status(2) || status(3))
            && exe::staff_is_transfer_listed_or_requested(p, club) == 0
            && ambition > 10
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 14 == 5
        {
            covered!("gra_2b leaving star talks");
            sd_media_outburst(d, 3, 0);
        }
    }
    for (bit, base) in [(0x200u32, 50), (0x400, 40), (0x80, 105)] {
        if g(contract.addr()) & bit != 0 && random(base - prof - loyalty) == 0 {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    if g(contract.addr()) & 0x10000 != 0 {
        staff_day_add_morale(contract.addr(), -5);
        // defect of the original: the second test, (grievances & 1) == 0x10000, can never hold
        if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
            exe::staff_contract_clear_grievances(contract, 0x10000);
        }
    }
    if g(contract.addr()) & 0x800000 != 0 {
        staff_day_add_morale(contract.addr(), -5);
        let future = |p: usize| exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0);
        if future(p) != 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
        } else if read_i32(p) % 14 == 0 && random(read_i8(p + staff::TEMPERAMENT) as i32 + 5) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
        } else if future(p) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
            staff_day_add_morale(contract.addr(), 15);
        }
    }
    if g(contract.addr()) & 0x4000 != 0 && random(100 - prof - loyalty) == 0 {
        exe::staff_contract_clear_grievances(contract, 0x4000);
    }
    if g(contract.addr()) & 0x8000 != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRAINING_MANAGER_HAS_FAST_POSITION_TRAINING);
        if f(read_usize(G_TRAINING_MANAGER), p) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x8000);
        }
    }
    if contract.squad_status() & 0xf < 10 {
        let n = ftol(read_f64(rdata::F64_100) - read_i8(p + staff::ADAPTABILITY) as f64 * read_f64(rdata::F64_3_5));
        if random(n) == 0 {
            covered!("gra_2b player learns the language");
            sd_learn_language(p, club.addr(), contract.addr());
            return;
        }
    }
    staff_day_homesick(d);
}

/// Adds `delta` to the contract's morale (`contract::MORALE`, signed byte), the sum held to -100..100: the clamp the
/// daily staff update repeats inline some sixty times (the sum in 32 bits; inside the range the byte gets
/// morale + delta).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), e.g. 0x5cf8ff..0x5cf920.
pub(crate) unsafe fn staff_day_add_morale(contract: usize, delta: i32) {
    let contract = Contract(contract);
    let m = contract.morale();
    let sum = (m as i32).wrapping_add(delta);
    if sum > 100 {
        contract.set_morale(100u8 as i8);
    } else if sum < -100 {
        contract.set_morale(0x9cu8 as i8);
    } else {
        contract.set_morale(((m as u8).wrapping_add(delta as u8)) as i8);
    }
}

pub(crate) unsafe fn sd_pow_double(base: f64, exponent: f64) -> f64 {
    let f: unsafe extern "C" fn(f64, f64) -> f64 = core::mem::transmute(FN_POW_DOUBLE_GRA);
    f(base, exponent)
}

/// One attribute byte of the player sheet moved by `delta` and held to `min..max` (`clamp_char_sum`; only `al` is
/// written back, as the original does).
pub(crate) unsafe fn sd_clamp_attribute(pd: usize, field: usize, delta: i32, min: i32, max: i32) {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CLAMP_CHAR_SUM_GRA);
    write_u8(pd + field, f(read_u8(pd + field) as u32, delta as u32, min as u32, max as u32) as u8);
}

/// A coach's rating for one player (0x5d4484..0x5d4540): his coaching (goalkeeping coaching for a goalkeeper 20),
/// plus random(3) when his motivating is above 15; with the player's professionalism under 6, -1 - random(5) when the
/// coach's discipline is under 6, else -1 under 15; with the player's pressure under 6 and the coach's discipline above
/// 10, -1 - random(5) when his man handling is under 6, else -1 under 15 (ratings by `non_player_attribute_rating`).
/// Without a player sheet the rating is 5.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4484..0x5d4540.
pub(crate) unsafe fn development_coach_rating(p: usize, np: usize) -> i8 {
    let p = Staff(p);
    use exe::utils_random as random;
    let pd = p.player();
    if pd.is_null() {
        return 5;
    }
    let coaching = if (pd.goalkeeper() as u8) == 20 { non_player::COACHING_GKS } else { non_player::COACHING };
    let mut rating = exe::non_player_attribute_rating(np, read_i8(np + coaching));
    if exe::non_player_attribute_rating(np, read_i8(np + non_player::MOTIVATING)) > 15 {
        rating = rating.wrapping_add(random(3) as i8);
    }
    if p.professionalism() < 6 {
        let discipline = read_i8(np + non_player::DISCIPLINE);
        if discipline < 6 {
            rating = rating.wrapping_add((-1 - random(5)) as i8);
        } else if discipline < 15 {
            rating = rating.wrapping_sub(1);
        }
    }
    if p.pressure() < 6 && read_i8(np + non_player::DISCIPLINE) > 10 {
        if exe::non_player_attribute_rating(np, read_i8(np + non_player::MAN_HANDLING)) < 6 {
            rating = rating.wrapping_add((-1 - random(5)) as i8);
        } else if exe::non_player_attribute_rating(np, read_i8(np + non_player::MAN_HANDLING)) < 15 {
            rating = rating.wrapping_sub(1);
        }
    }
    rating
}

/// An attribute byte under 100 (and above 0 when `positive`) becomes ftol of the larger of × 1.1 and + 5.
pub(crate) unsafe fn sd_grow_ten_percent(pd: usize, field: usize, positive: bool) {
    let v = read_i8(pd + field);
    if v >= 100 || (positive && v <= 0) {
        return;
    }
    let tenth = v as f64 * read_f64(rdata::F64_1_1);
    let five = (v as i32 + 5) as f64;
    write_u8(pd + field, ftol(if tenth > five { tenth } else { five }) as u8);
}

/// An attribute byte loses one when it is above `above` and random(3) is 0.
pub(crate) unsafe fn sd_maybe_lose_one(pd: usize, field: usize, above: i8) {
    if read_i8(pd + field) > above && exe::utils_random(3) == 0 {
        write_u8(pd + field, read_u8(pd + field).wrapping_sub(1));
    }
}

/// An attribute byte loses random(`n`) when it is above `high`, else one when it is above `low` and random(3) is 0.
pub(crate) unsafe fn sd_lose_some(pd: usize, field: usize, high: i8, n: i32, low: i8) {
    if read_i8(pd + field) > high {
        write_u8(pd + field, read_u8(pd + field).wrapping_sub(exe::utils_random(n) as u8));
    } else {
        sd_maybe_lose_one(pd, field, low);
    }
}

/// A forward (or attacking midfielder) rated 20 past `ability_over` loses random(`n`) of current ability when
/// random(ability) beats professionalism + stamina + `margin`.
pub(crate) unsafe fn sd_veteran_ability_loss(p: usize, pd: usize, ability_over: i16, margin: i32, n: i32) {
    let pd = PlayerSheet(pd);
    let p = Staff(p);
    let ability = pd.current_ability();
    if ability <= ability_over {
        return;
    }
    if exe::utils_random(ability as i32) > p.professionalism() as i32 + pd.stamina() as i32 + margin {
        let loss = exe::utils_random(n);
        pd.set_current_ability(((pd.current_ability() as u16).wrapping_sub(loss as u16)) as i16);
    }
}

pub(crate) unsafe fn sd_contract_satisfaction(contract: usize, p: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_CONTRACT_SATISFACTION);
    f(contract, p)
}

pub(crate) unsafe fn sd_post_wants_new_contract_news(contract: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_WANTS_NEW_CONTRACT_NEWS);
    f(read_usize(G_CONTRACT_MANAGER), contract)
}

pub(crate) unsafe fn sd_count_disliked_at_club(p: usize, club: usize) -> u8 {
    let f: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB);
    f(p, club)
}

pub(crate) unsafe fn sd_count_favoured_at_club(p: usize, club: usize) -> u8 {
    let f: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB);
    f(p, club)
}

/// The club of the contract (`contract::CLUB`, an id into `G_CLUB_TABLE`).
pub(crate) unsafe fn sd_contract_club(contract: usize) -> usize {
    let contract = Contract(contract);
    club_at((contract.club() as u32) as isize).addr()
}

/// Today against the contract's end (`Date::days_since` from the end date): days left.
pub(crate) unsafe fn sd_days_to_expiry(contract: usize) -> i32 {
    let contract = Contract(contract);
    exe::date_days_since(contract.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP))
}

/// `CONTRACT_MANAGER::offer_contract` of the AI club for a new contract (no club, no output, slot -1, factor 1).
pub(crate) unsafe fn sd_offer_new_contract(p: usize, mode: bool) -> u32 {
    exe::contract_manager_offer_contract(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 1.0, mode as u32)
}

/// No offer came: without grievances he asks for a new contract (grievance 2), with some he loses 5 morale
/// (0x5d19e4, 0x5d1bc1).
pub(crate) unsafe fn sd_ask_for_contract(d: &StaffDay) {
    if read_u32(d.contract + contract::GRIEVANCES) != 0 {
        staff_day_add_morale(d.contract, -5);
    } else {
        covered!("gra_2b player asks for a new contract");
        exe::staff_contract_apply_event_flags(d.contract, 2, 0);
    }
}

/// A contented player who already asked (grievance 2, 0x5d1d46..0x5d1eef): one day in 45 the AI club offers again
/// (+15 morale when it does); one player in 8 with grievance 2 alone and no offer pending goes public
/// (`post_wants_new_contract_news`, may dislike the club or manager); he loses 10 morale.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1d46..0x5d1eef.
pub(crate) unsafe fn staff_day_contract_reminder(d: &StaffDay, entry: usize) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let contract = Contract(d.contract);
    if exe::human_manager_is_human_club(sd_contract_club(contract.addr()), 0, 1, 0, 0) == 0
        && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p.addr())) % 45 == 0
    {
        let s = (read_u8(entry + 0x24) >> 1) & 0xf;
        let offered = match s {
            1..=3 => sd_offer_new_contract(p.addr(), read_u32(entry + 0xf) == read_u32(p.addr())),
            4..=6 => sd_offer_new_contract(p.addr(), false),
            _ => 0,
        };
        if offered != 0 {
            staff_day_add_morale(contract.addr(), 15);
        }
    }
    let id = read_u32(p.addr());
    if id & 7 == 0
        && contract.grievances() == 2
        && exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), id, contract.club() as u32) == 0
    {
        covered!("gra_2b player reminds the club publicly");
        sd_post_wants_new_contract_news(contract.addr());
        if read_i16(d.pd + player_sheet::CURRENT_REPUTATION) > 0x1482
            && p.ambition() > 5
            && p.temperament() < 15
            && p.sportsmanship() < 15
            && p.professionalism() < 10
            && random(14) == 0
        {
            sd_dislike_club_or_manager(d);
        }
    }
    staff_day_add_morale(contract.addr(), -10);
}

/// A restless player goes to the media: with a story already running, random(professionalism) under random(6), else
/// one time in professionalism + `extra` (`extra` 5 for kind 1, 0 for kind 3).
pub(crate) unsafe fn sd_media_outburst(d: &StaffDay, kind: u32, extra: i32) {
    use exe::utils_random as random;
    let prof = read_i8(d.p + staff::PROFESSIONALISM) as i32;
    if exe::media_has_story_for_player(read_usize(G_MEDIA), d.p) != 0 {
        let a = random(prof);
        if a < random(6) {
            sd_media_story(d, kind);
            return;
        }
    }
    if random(prof + extra) == 0 {
        sd_media_story(d, kind);
    }
}

/// The people the player dislikes (`staff_get_disliked_staff`, up to 50) and whether one shares his club; the
/// first one found stops the search.
pub(crate) unsafe fn sd_disliked_team_mate(p: usize) -> (bool, u8) {
    let mut list = [0usize; 50];
    let mut count: u8 = 0;
    exe::staff_get_disliked_staff(p, list.as_mut_ptr() as usize, &mut count as *mut u8 as usize);
    let mut k: i8 = 0;
    while k < count as i8 {
        let other = list[k as usize];
        if other != 0 && sd_club_of(other) == sd_club_of(p) {
            return (true, count);
        }
        k += 1;
    }
    (false, count)
}

/// The squad status nibble (here the language level at the club) rises by one, at most 10; at 1 the player adds the
/// club nation's first language (`PlayerStats::add_language`), up to 10 he improves it (`improve_language`).
pub(crate) unsafe fn sd_learn_language(p: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let club = Club(club);
    let s = contract.squad_status();
    if s & 0xf < 10 {
        contract.set_squad_status(((s.wrapping_add(1) ^ s) & 0xf) ^ s);
    }
    let level = contract.squad_status() & 0xf;
    let language = read_u8(club.nation().addr() + nation::FIRST_LANGUAGE) as u32;
    if level == 1 {
        exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
    } else if level <= 10 {
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_IMPROVE_LANGUAGE);
        f(read_usize(G_PLAYER_STATS), p, language, 1, 1);
    }
}

/// Homesickness (0x5d304e..0x5d352d): a player abroad (neither nation is the club's) with adaptability under 10, one
/// day in 30: the days in the contract, halved for a star (world reputation above 7000) on his own continent or for a
/// player from outside Europe [inference: `KEY_CONTINENT_EUROPE`], × 0.8 with a compatriot favoured team mate. Outside
/// his home region, past (professionalism + loyalty + 16) × 50 + (adaptability - age) × 20 + id % 200 days, he loses 5
/// morale and becomes homesick (grievance 0x20000) unless already unsettled (0x20000 or 0x20). Otherwise, between 50
/// and 500 days, when squad status² × 15 + morale × 1.5 - days falls under -150: grievance 0x20 with -50 morale, or
/// with it already, -10 morale and, for a disloyal, famous, very unhappy player one day in 12 × pressure, an absence.
/// A settled squad status (4 or more) ends grievance 0x20.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d304e..0x5d352d.
pub(crate) unsafe fn staff_day_homesick(d: &StaffDay) {
    use exe::utils_random as random;
    let p = Staff(d.p);
    let club = Club(d.club);
    let contract = Contract(d.contract);
    let club_nation = club.nation().addr();
    if club_nation == p.nation().addr()
        || club_nation == p.second_nation().addr()
        || p.adaptability() >= 10
        || random(30) != 0
    {
        return;
    }
    let mut days = sd_days_since_inline(contract.addr() + contract::DATE_STARTED);
    let home = p.nation();
    let same_continent = read_i16(d.pd + player_sheet::WORLD_REPUTATION) > 0x1b58
        && read_usize(club.nation().addr() + nation::CONTINENT) == home.continent();
    let continent = home.continent();
    if same_continent || (continent != 0 && read_u32(continent) != read_u32(KEY_CONTINENT_EUROPE)) {
        days /= 2;
    }
    if d.mate != 0 && read_usize(d.mate + staff::NATION) == home.addr() {
        days = ftol(days as f64 * read_f64(rdata::F64_0_8));
    }
    if home.region() != read_u8(club.nation().addr() + nation::REGION) {
        let character = (p.professionalism() as i32 + p.loyalty() as i32 + 16) * 50
            + (p.adaptability() as i32 - (p.age() as i8) as i32) * 20
            + read_i32(p.addr()) % 200;
        if days > character {
            let g = contract.grievances();
            if g & 0x20000 == 0 && g & 0x20 == 0 {
                covered!("gra_2b player homesick");
                staff_day_add_morale(contract.addr(), -5);
                exe::staff_contract_apply_event_flags(contract, 0x20000, 0);
            }
            return;
        }
    }
    if days > 50 && days < 500 {
        let s = (contract.squad_status() & 0xf) as i32;
        let morale = contract.morale();
        let mood = (s * s * 15) as f64 + morale as f64 * read_f64(rdata::F64_1_5) - days as f64;
        if !(mood >= read_f64(rdata::F64_NEG_150)) {
            // under -150
            if contract.grievances() & 0x20 == 0 {
                covered!("gra_2b player unsettled abroad");
                staff_day_add_morale(contract.addr(), -50);
                exe::staff_contract_apply_event_flags(contract, 0x20, 0);
                return;
            }
            staff_day_add_morale(contract.addr(), -10);
            if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
                || exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
                || p.loyalty() >= 5
                || random(12) != 0
                || random(p.pressure() as i32) != 0
                || p.player().addr() == 0
                || read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) <= 0xcb2
                || contract.morale() >= -50
            {
                return;
            }
            covered!("gra_2b homesick player goes absent");
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE_GRA);
            f(read_usize(G_AWOL), p.addr(), 0, 0);
            return;
        }
    }
    if contract.squad_status() & 0xf >= 4
        && contract.grievances() & 0x20 != 0
        && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
    {
        exe::staff_contract_clear_grievances(contract, 0x20);
    }
}

/// A media story about the player at his club (`Media::create_player_media_story`, no extra arguments).
pub(crate) unsafe fn sd_media_story(d: &StaffDay, kind: u32) {
    exe::media_create_player_media_story(read_usize(G_MEDIA), d.p, d.club, kind, 0, 0);
}

/// The club of a person's contract, unless it is of type 5 [inference: a trial] (0 without one); the lookup is kept in
/// `G_CONTRACT_LOOKUP_RESULT` as the original does.
pub(crate) unsafe fn sd_club_of(person: usize) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if !c.is_null() && c.contract_type() & 0x3f != 5 {
        sd_contract_club(c.addr())
    } else {
        0
    }
}

/// `staff_pick_favoured_team_mate` (cdecl: person, club).
pub(crate) const FN_STAFF_PICK_FAVOURED_TEAM_MATE: usize = 0x548da0;

/// `Dispute::remove_dispute` (thiscall: person).
pub(crate) const FN_DISPUTE_REMOVE_DISPUTE: usize = 0x56a090;

/// `STAFF_CONTRACT::get_contract_satisfaction` (thiscall: person; 1 to 3).
pub(crate) const FN_STAFF_CONTRACT_GET_CONTRACT_SATISFACTION: usize = 0x866620;

/// `CONTRACT_MANAGER::post_wants_new_contract_news` (thiscall: contract).
pub(crate) const FN_CONTRACT_MANAGER_POST_WANTS_NEW_CONTRACT_NEWS: usize = 0x4d9f80;

/// `club_reputation_for_staff` (cdecl: club, person).
pub(crate) const FN_CLUB_REPUTATION_FOR_STAFF: usize = 0x545e90;

/// `staff_count_disliked_staff_at_club` (cdecl: person, club).
pub(crate) const FN_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB: usize = 0x547c90;

/// `staff_count_favoured_staff_at_club` (cdecl: person, club).
pub(crate) const FN_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB: usize = 0x547ce0;

/// `Media::queue_wants_first_team_story` (thiscall: player, reply round).
pub(crate) const FN_MEDIA_QUEUE_WANTS_FIRST_TEAM_STORY: usize = 0x75a640;

/// `TRAINING_MANAGER::has_fast_position_training` (thiscall: person).
pub(crate) const FN_TRAINING_MANAGER_HAS_FAST_POSITION_TRAINING: usize = 0x8b4db0;

/// `PlayerStats::improve_language` (thiscall: person, language, raise, sort).
pub(crate) const FN_PLAYERSTATS_IMPROVE_LANGUAGE: usize = 0x7c6ca0;

/// The locals of one person's day in `game_daily_staff_update`, as the original keeps them in its frame.
#[derive(Clone, Copy)]
pub(crate) struct StaffDay {
    /// The person (`ebp`, [esp+0x6c]).
    pub(crate) p: usize,
    /// The person's id times the state record size ([esp+0x28]).
    pub(crate) state_off: usize,
    /// The player sheet ([esp+0x40]).
    pub(crate) pd: usize,
    /// The club the person plays for ([esp+0x1c]): the contract's club, or the club job for jobs 5 and 12.
    pub(crate) club: usize,
    /// The contract with that club ([esp+0x68], `esi`).
    pub(crate) contract: usize,
    /// The favoured team mate (`staff_pick_favoured_team_mate`, [esp+0x5c]).
    pub(crate) mate: usize,
    /// The club job differs from the club ([esp+0x27]).
    pub(crate) away_from_job: bool,
    /// The odds of the day's unrest roll ([esp+0x20]; only its low 16 bits are ever read).
    pub(crate) odds: i32,
    /// The club job's linked B team ([esp+0x7c]).
    pub(crate) b_team: u32,
}

/// The routine's locals that outlive one person.
pub(crate) struct StaffDayRun {
    /// [esp+0x70]: the favoured team mate's bonus, a short the original never initialises and carries from one person
    /// to the next (defect of the original: the first one reads stack residue; the copy starts it at 0).
    pub(crate) slot70: u32,
}

/// `news_post_training_report` (cdecl: player, coach, club, praise).
pub(crate) const FN_NEWS_POST_TRAINING_REPORT: usize = 0x588cd0;

/// `dispute_post_news` (stdcall in the map: staff, club, outcome, reason a, reason b; the original also sets ecx).
pub(crate) const FN_DISPUTE_POST_NEWS_GRA: usize = 0x56a3b0;

/// `Media::post_player_praise_story` (thiscall: player, club, topic, fixture record or null).
pub(crate) const FN_MEDIA_POST_PLAYER_PRAISE_STORY: usize = 0x744240;

/// `PlayerStats::get_last_match_rating` (thiscall, the player sheet; the rating on x87, 6.5 without one).
pub(crate) const FN_PLAYERSTATS_GET_LAST_MATCH_RATING: usize = 0x7c1a60;

/// `news_post_player_thanks_manager` (cdecl: player, club).
pub(crate) const FN_NEWS_POST_PLAYER_THANKS_MANAGER: usize = 0x589300;

/// The ratings the club stories read (0x5d06b7..0x5d0838), as the original keeps them (floats).
pub(crate) struct ClubStoryRatings {
    /// The game's average rating ([esp+0x20]).
    pub(crate) average: f32,
    /// The season rating (stat 6/1, [esp+0x2c]).
    pub(crate) season: f32,
    /// The other season rating plus the club's standing bonus ([esp+0x28]).
    pub(crate) other: f32,
    /// The recent form × 0.2 ([esp+0x58]).
    pub(crate) form: f32,
    /// Stat 6/2 over the season rating (at least 1), [esp+0x14].
    pub(crate) ratio: f32,
    /// The last match rating ([esp+0x44]).
    pub(crate) last: f32,
}

/// `CONTRACT_MANAGER::post_seeks_assurances_news` (thiscall: the contract).
pub(crate) const FN_CONTRACT_MANAGER_POST_SEEKS_ASSURANCES_NEWS: usize = 0x4d9840;

/// `AWOL::start_absence` (thiscall: player, two zeros).
pub(crate) const FN_AWOL_START_ABSENCE_GRA: usize = 0x419840;

/// `Injury::get_match_condition` (0x62c510, thiscall: player, in percent).
pub(crate) const FN_INJURY_GET_MATCH_CONDITION_GRA: usize = 0x62c510;

/// `AWOL::report_poor_fitness` (0x41add0, thiscall: player, club).
pub(crate) const FN_AWOL_REPORT_POOR_FITNESS_GRA: usize = 0x41add0;

/// `staff_remove_favoured_club` (0x547f50, cdecl: person, club).
pub(crate) const FN_STAFF_REMOVE_FAVOURED_CLUB_GRA: usize = 0x547f50;

/// The game.cpp source path (the error box).
pub(crate) const GAME_CPP_PATH: usize = 0x9cd2a4;

/// 1 until the day's first development builds the two ability tables (`G_PLAYER_ABILITY_VALUES_SORTED`,
/// `G_PLAYER_DATA_PTRS_BY_ABILITY`); byte.
pub(crate) const G_ABILITY_TABLES_PENDING: usize = 0x9cd236;

/// qsort comparer of two 16-bit abilities (game.cpp, cdecl).
pub(crate) const FN_COMPARE_ABILITY_VALUES: usize = 0x5d6200;

/// qsort comparer of two player data records by ability, then potential (`compare_players_by_ability`, hooked).
pub(crate) const FN_COMPARE_PLAYERS_BY_ABILITY: usize = 0x5d6220;

/// The 13 growth steps of a development day (shorts, -15..14), indexed by two ratings held to 0..12.
pub(crate) const DEVELOPMENT_GROWTH_STEPS: usize = 0x9cd21c;

/// f32 -3.0: below it a falling player loses reputation at the fixed rates.
pub(crate) const F32_NEG_3_DEVELOPMENT: usize = 0x96afc0;

/// 7.5e-5: the potential squared, as part of the first rating.
pub(crate) const F64_POTENTIAL_SQUARED_SCALE: usize = 0x96af98;

/// 6.5: the base of the second rating.
pub(crate) const F64_6_5_DEVELOPMENT: usize = 0x96af90;

/// 1 / 26.6: a club's training facilities (or a manager's youngsters rating) to the training factor.
pub(crate) const F64_TRAINING_SCALE: usize = 0x96af80;

/// 1 / 13,300: a club's reputation to the training factor when it has no training facilities rating.
pub(crate) const F64_CLUB_REPUTATION_TRAINING_SCALE: usize = 0x96af78;

/// f32 1.31: the training factor at a club with a human manager (or none).
pub(crate) const F32_1_31_HUMAN_MANAGER: usize = 0x96af70;

/// 1 / 53.2: a coach's rating to the training factor.
pub(crate) const F64_COACH_SCALE: usize = 0x96af68;

/// 1 / 30.7: the division's reputation to the level factor.
pub(crate) const F64_DIVISION_LEVEL_SCALE: usize = 0x96af60;

/// 1 / 15,350: the club's reputation to the level factor when it has no division.
pub(crate) const F64_CLUB_LEVEL_SCALE: usize = 0x96af58;

/// -0.9: the share of the ability kept above a weak division's ceiling.
pub(crate) const F64_NEG_0_9_DEVELOPMENT: usize = 0x96af50;

/// `pow_double` (0x5d5a20, game.cpp, cdecl: base, exponent; the result in st0).
pub(crate) const FN_POW_DOUBLE_GRA: usize = 0x5d5a20;

/// `game_average` (0x5d5a60, game.cpp, thiscall: the player stats, second; the result in st0).
pub(crate) const FN_GAME_AVERAGE_GRA: usize = 0x5d5a60;

/// `clamp_char_sum` (0x5d61d0, game.cpp, cdecl: value, delta, minimum, maximum; bytes).
pub(crate) const FN_CLAMP_CHAR_SUM_GRA: usize = 0x5d61d0;

/// `player_search_get_club_target_rating` (0x7bca90, cdecl: club, base, forced).
pub(crate) const FN_PLAYER_SEARCH_GET_CLUB_TARGET_RATING_GRA: usize = 0x7bca90;

/// `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall: the player).
pub(crate) const FN_TRAINING_MANAGER_GET_PLAYER_ENTRY_GRA: usize = 0x5d5ac0;

/// `staff_fade_staff_relationships` (0x547f90, cdecl: person).
pub(crate) const FN_STAFF_FADE_STAFF_RELATIONSHIPS_GRA: usize = 0x547f90;

/// `Injury::daily_wear` (0x62c2f0, thiscall: the person's id).
pub(crate) const FN_INJURY_DAILY_WEAR_GRA: usize = 0x62c2f0;
