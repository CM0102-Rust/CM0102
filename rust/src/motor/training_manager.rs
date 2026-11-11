use super::*;

/// Staff id -> person record (0x6e bytes in G_STAFF_TABLE).
pub(crate) unsafe fn training_manager_staff_by_id(id: i32) -> usize {
    staff_at(id as isize).addr()
}

/// Resets a 9-byte coaching slot to empty (id -1, ratings 0).
pub(crate) unsafe fn clear_coach_slot(e: usize) {
    write_i32(e, -1);
    write_u32(e + 4, 0);
    write_u8(e + 8, 0);
}

/// The rating divisors by the coach's number of categories (1 to 5), floats built on the stack by the original: 1.0;
/// 1.2; 1.4; 1.6; 2.0.
pub(crate) const RATING_DIVISORS: [u32; 5] = [0x3f80_0000, 0x3f99_999a, 0x3fb3_3333, 0x3fcc_cccd, 0x4000_0000];

/// Non-player attribute rating (`exe::non_player_attribute_rating`) for the attribute byte at f + k.
pub(crate) unsafe fn non_player_attribute_rating(f: usize, k: usize) -> i32 {
    exe::non_player_attribute_rating(f, read_i8(f + k)) as i8 as i32 // the original routine sign-extends al (movsx)
}

/// `TRAINING_MANAGER::update_coach_assignments(club, tecnico)`, called by the coach hirings and departures
/// (`exe::contract_manager_resolve_contract_offers`, `sign_contract`, `exe::transfer_manager_move_player_to_club`, the
/// regen `player_regen_retire_from_non_playing_staff`) and by the club change (`reassign_squad_schedules`): updates the
/// six training staff slots of the human manager and recomputes each category rating.
///
/// Without a manager and with a club without a human (`exe::human_manager_is_human_club` with 1, 1, 0, 0), or a
/// national side club (`exe::database_is_national_team`): nothing. A null manager: the club's (`.manager()`). The
/// human's coach assignments (`exe::training_manager_get_coach_assignments`, six 9-byte slots): (1) the slot whose
/// coach has no non-player sheet (`staff::NON_PLAYER`) or is neither the assistant (`exe::database_get_club_assistant`)
/// nor one of the 5 coaches (`exe::database_get_club_coaches`) is emptied; (2) the assistant and the coaches with a
/// sheet that are not in the slots go into the first empty one with all five categories (0x01010101 and 1); (3) each
/// flagged category gets its rating: 0 (goalkeepers, sheet +0x10) (chance − sheet `non_player::COACHING_TECHNIQUE` +
/// 21) / 2; 1 (tactics, +0x21 and +0x10) (chance + chance + `non_player::COACHING_TECHNIQUE`) / 3; 2 (+0x10) (chance +
/// `non_player::ATTACKING` + `non_player::COACHING_TECHNIQUE`) / 3; 3 (+0x10) (chance +
/// `non_player::COACHING_TECHNIQUE`) / 2; 4 (+0x11) the chance (divisions truncated toward zero; the chance from
/// `non_player_attribute_rating`), the signed byte divided by the divisor for the number of flagged categories (1.0 to
/// 2.0, float), truncated and clamped to 1..20.
/// Original: `TRAINING_MANAGER::update_coach_assignments` (0x8b2880, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_coach_assignments(this: usize, club: usize, tecnico: usize) {
    let club = Club(club);
    if tecnico == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let manager = if tecnico == 0 { club.manager().addr() } else { tecnico };
    let t = exe::training_manager_get_coach_assignments(this, manager);
    let aux = exe::database_get_club_assistant(club);
    let mut tr = [0usize; 5];
    exe::database_get_club_coaches(club, tr.as_mut_ptr());
    let candidate = |k: i32| if k < 0 { aux } else { tr[k as usize] };
    for v in 0..6 {
        let e = t + 9 * v;
        let id = read_i32(e);
        if id == -1 {
            continue;
        }
        if read_usize(training_manager_staff_by_id(id) + staff::NON_PLAYER) == 0 {
            clear_coach_slot(e);
            continue;
        }
        if !(-1..5).any(|k| {
            let p = candidate(k);
            p != 0 && read_i32(p) == id
        }) {
            clear_coach_slot(e);
        }
    }
    for k in -1..5 {
        let p = candidate(k);
        if p == 0 || read_usize(p + staff::NON_PLAYER) == 0 {
            continue;
        }
        if (0..6).any(|v| read_i32(t + 9 * v) == read_i32(p)) {
            continue;
        }
        if let Some(v) = (0..6).find(|&v| read_i32(t + 9 * v) == -1) {
            let e = t + 9 * v;
            write_i32(e, read_i32(p));
            write_u32(e + 4, 0x0101_0101);
            write_u8(e + 8, 1);
        }
    }
    for v in 0..6 {
        let e = t + 9 * v;
        let id = read_i32(e);
        if id == -1 {
            continue;
        }
        let f = read_usize(training_manager_staff_by_id(id) + staff::NON_PLAYER);
        if f == 0 {
            clear_coach_slot(e);
            continue;
        }
        let n = (0..5).filter(|&j| read_u8(e + 4 + j) != 0).count();
        for j in 0..5 {
            if read_u8(e + 4 + j) == 0 {
                continue;
            }
            let b = |k: usize| read_i8(f + k) as i32;
            let x = match j {
                0 => (non_player_attribute_rating(f, 0x10) - b(non_player::COACHING_TECHNIQUE) + 0x15) / 2, // cqf7: the goalkeeping rating
                1 => {
                    let a = non_player_attribute_rating(f, 0x21);
                    (a + non_player_attribute_rating(f, 0x10) + b(non_player::COACHING_TECHNIQUE)) / 3 // cqf7: the tactics rating
                }
                2 => (non_player_attribute_rating(f, 0x10) + b(non_player::ATTACKING) + b(non_player::COACHING_TECHNIQUE)) / 3,
                3 => (non_player_attribute_rating(f, 0x10) + b(non_player::COACHING_TECHNIQUE)) / 2,
                _ => non_player_attribute_rating(f, 0x11),
            };
            // the low byte, signed, as float divided by the float of the quantity (x87 double precision)
            let q = ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[n - 1]) as f64) as i8; // cqf7: the quantity divisor
            covered!("cqf_7 coach rating");
            write_i8(e + 4 + j, q.clamp(1, 20));
        }
    }
}

/// Training schedule `vaga` (0..9, else 1) of human manager `tec`; outside the 16 humans shows the error box and
/// returns 0.
pub(crate) unsafe fn human_training_schedule(this: usize, manager: usize, slot: i8) -> usize {
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&slot) { slot as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// 6-byte training entry of player data `f` ([this+0x651d] + 6 x id).
pub(crate) unsafe fn training_entry_of_player_data(this: usize, f: usize) -> usize {
    read_usize(this + training_manager_fields::PLAYER_ENTRIES).wrapping_add((read_i32(f) as isize * 6) as usize)
}

/// `TRAINING_MANAGER::reassign_squad_schedules(manager, old, new)`, when the human manager changes club
/// (`remove_staff_from_club` and 0x68f9c0/0x69ebc0, manager_manager.cpp): reassigns the training schedules of the old
/// and new squads.
///
/// The old club's squad (`exe::database_collect_club_squad`, 50): each valid person (id below `G_STAFF_COUNT`) gets
/// load 50 (`exe::injury_set_training_load`) and slot 6 (goalkeeper with 15 or more, sheet `player_sheet::GOALKEEPER`)
/// or 1 in the training entry; without a sheet, the box (line 0x34c). With a new club: of the manager's ten schedules,
/// the last one of type 6 (`training_schedule::TYPE`) is the goalkeepers' and the last one of type 1 the general one;
/// each player of the new squad with a sheet goes to the goalkeepers' schedule (goalkeeper with 15 or more, if there is
/// one), otherwise the general one (if there is one), otherwise slot 1, with the schedule's load
/// (`exe::training_schedule_get_training_load`) on the fitness sheet; finally the training staff
/// (`FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS` with the new club and the manager).
/// Original: `TRAINING_MANAGER::reassign_squad_schedules` (0x8b2470, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reassign_squad_schedules(this: usize, manager: usize, old: usize, new: usize) {
    let (mut gol, mut geral) = (-1i8, -1i8);
    if old != 0 {
        let mut v = [0usize; 50];
        exe::database_collect_club_squad(old, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            exe::injury_set_training_load(read_usize(G_INJURY), p, 0x32);
            let f = read_usize(p + staff::PLAYER);
            if f == 0 {
                show_error_at(0xa8ccd0, 0x34c);
                continue;
            }
            write_u8(training_entry_of_player_data(this, f), if read_i8(f + player_sheet::GOALKEEPER) >= 0xf { 6 } else { 1 });
        }
    }
    if new == 0 {
        return;
    }
    for vaga in 0..10i8 {
        if read_u8(human_training_schedule(this, manager, vaga) + training_schedule::TYPE) == 6 {
            gol = vaga;
        } else if read_u8(human_training_schedule(this, manager, vaga) + training_schedule::TYPE) == 1 {
            geral = vaga;
        }
    }
    let mut v = [0usize; 50];
    exe::database_collect_club_squad(new, v.as_mut_ptr(), 0);
    for &p in &v {
        if p == 0 {
            continue;
        }
        let f = read_usize(p + staff::PLAYER);
        if f == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let e = training_entry_of_player_data(this, f) as *mut i8;
        *e = if read_i8(f + player_sheet::GOALKEEPER) >= 0xf && gol != -1 {
            gol // cqf7: the goalkeepers programme
        } else if geral != -1 {
            geral
        } else {
            1
        };
        covered!("cqf_7 new squad schedule");
        let c = exe::training_schedule_get_training_load(human_training_schedule(this, manager, *e));
        exe::injury_set_training_load(read_usize(G_INJURY), p, c as u32);
    }
    let staff: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
    staff(this, new, manager);
}

/// the default names of the 167 training schedules
/// Original: `(training_manager.cpp, 0x8b2230)` (0x8b2230, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn training_manager_cpp_0x8b2230(this: usize) {
    for k in 0..0xa7 {
        exe::training_schedule_set_default_name(this + k * 0x9b);
    }
}

/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
pub(crate) unsafe fn training_entry_of_staff(this: usize, p: Staff) -> usize {
    read_usize(this + training_manager_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as isize * 6) as usize)
}

/// Person -> contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
pub(crate) unsafe fn training_manager_get_contract_stored(p: Staff) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// Contract -> club record (id +4), or 0 without contract.
pub(crate) unsafe fn training_manager_club_of_contract(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        club_at(read_i32(c + 4) as isize).addr()
    }
}

/// Manager's id is among the last 16 staff ids (the human managers).
pub(crate) unsafe fn is_human_manager_id(manager: usize) -> bool {
    read_i32(manager) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10)
}

/// `TRAINING_MANAGER::update_player_training(p, skip)`, called by the player initialisation for the match
/// (`exe::match_player_setup`, match_man.cpp, with 0: every player of every match), by a new injury (`add_injury`) and
/// by the screens: brings the player's training weeks up to date (up to 30).
///
/// A null person (line 0x18a), without the sheet (0x195) or without a schedule
/// (`exe::training_manager_get_player_schedule` null, 0x1a3): the "Error" box; an id from `G_STAFF_COUNT` on: nothing.
/// A player of a human manager's club without flag 0x20 in +5 of the training entry: the flag, the schedule slot (the
/// goalkeepers' one, type 6, for a goalkeeper with 15 or more; otherwise the general one, type 1; otherwise 1) and its
/// load (`exe::training_schedule_get_training_load`) on the fitness sheet (`exe::injury_set_training_load`). Weeks =
/// training week (`training_manager_fields::WEEK_COUNTER`) − the entry's (+1); none and without `skip`: nothing. With
/// the flag and the club no longer with a human manager: clears the flag, load 50, slot 6 or 1 and the position
/// training stops (`exe::training_manager_set_position_training_rate` with 0).
///
/// Up to 30 weeks: brings the entry up to date; each category (0..4) has the weight level × 2 + 60 + (coach factor −
/// 100) / 5 (0..200) and, per week, progress rolls: up 1 with weight above 100, draw(210 − weight) < 20, value below
/// (`.determination()`) / 3 and draw(|value| × 2 + 14) < 10; otherwise down 1 with weight below 110, draw(weight + 60)
/// < 20, value above −7 and above (`.professionalism()`) / 3 − 6 and the same last draw (goalkeeper with 19 or more:
/// only with draw(4), or draw(2) in category 4, at zero).
///
/// The position training items (`exe::training_manager_find_position_training`, 8-byte items: +4 the mask, +6 the
/// rating, +7 the rate) advance per week (or once, with `skip` in the mask): with a positive rate, draw(rate) < 5 gives
/// rating + 2 (up to 20) and the player may complain (club contract `FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB` without
/// `grievances::UNHAPPY_WITH_TRAINING` in `contract::GRIEVANCES`: by a rate above 50 or an aptitude
/// `exe::training_manager_get_position_aptitude` below 12, with `.professionalism()` below 15,
/// draw(`.professionalism()`) < 10 and no injury `exe::injury_get_player_injury_status`: flag 0x8000
/// (`exe::staff_contract_apply_event_flags`), −5 on `contract::MORALE` (−100..100) and
/// `exe::news_post_player_unhappy_with_training`); with a negative rate, draw(−rate) < 5 gives rating − 2 and, below 1,
/// the item ends (rate 0, rating 1). It applies the progress (`exe::training_manager_apply_training_progress`) and,
/// with finished items, removes them from the list (the last one into the gap) and re-sorts (qsort `exe::crt_qsort`
/// with 0x8b4e00).
/// Original: `TRAINING_MANAGER::update_player_training` (0x8b1750, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_player_training(this: usize, p: usize, skip: u32) {
    let p = Staff(p);
    if p.is_null() {
        show_error_at(0xa8ccd0, 0x18a);
        return;
    }
    if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if p.player().is_null() {
        show_error_at(0xa8ccd0, 0x195);
        return;
    }
    let mut d = [0i8; 8]; // the progress of the five categories (the original clears a dword and a byte)
    let e = training_entry_of_staff(this, p);
    let prog = exe::training_manager_get_player_schedule(this, p);
    if prog == 0 {
        show_error_at(0xa8ccd0, 0x1a3);
        return;
    }
    let mark = (e + 5) as *mut u8;
    if *mark & 0x20 == 0 {
        let c = training_manager_get_contract_stored(p);
        if c != 0 && training_manager_club_of_contract(c) != 0 && read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER) != 0
            && is_human_manager_id(read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER)) {
            let manager = read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER);
            *mark |= 0x20;
            let (mut goleiro, mut geral) = (-1i8, -1i8);
            for k in 0..10i8 {
                if read_u8(human_training_schedule(this, manager, k) + training_schedule::TYPE) == 6 {
                    goleiro = k;
                } else if read_u8(human_training_schedule(this, manager, k) + training_schedule::TYPE) == 1 {
                    geral = k;
                }
            }
            let slot = if p.player().goalkeeper() >= 0xf && goleiro != -1 {
                goleiro
            } else if geral != -1 {
                geral
            } else {
                1
            };
            write_i8(e, slot);
            let v = exe::training_schedule_get_training_load(human_training_schedule(this, manager, slot));
            covered!("cqd_8 human schedule");
            exe::injury_set_training_load(read_usize(G_INJURY), p, v as u32);
        }
    }
    let mut weeks = read_u16(this + training_manager_fields::WEEK_COUNTER).wrapping_sub(read_u16(e + 1));
    let skip = skip as u16;
    if weeks == 0 && skip == 0 {
        return;
    }
    if *mark & 0x20 != 0 {
        let c = training_manager_get_contract_stored(p);
        let ainda = c != 0 && training_manager_club_of_contract(c) != 0 && read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER) != 0
            && is_human_manager_id(read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER));
        if !ainda {
            covered!("cqd_8 left the human club");
            *mark &= 0xdf;
            exe::injury_set_training_load(read_usize(G_INJURY), p, 0x32);
            let f = p.player();
            write_u8(training_entry_of_staff(this, p), if f.goalkeeper() >= 0xf { 6 } else { 1 });
            let mut q = 0usize;
            let n = exe::training_manager_find_position_training(this, p, &mut q);
            let mut k = 0i8;
            if n > 0 {
                loop {
                    exe::training_manager_set_position_training_rate(this, q.wrapping_add((k as isize * 8) as usize), 0);
                    k = k.wrapping_add(1);
                    if k as i32 >= n {
                        break;
                    }
                }
            }
        }
    }
    if weeks > 0x1e {
        weeks = 0x1e;
    }
    write_u16(e + 1, read_u16(this + training_manager_fields::WEEK_COUNTER));
    let mut q = 0usize;
    let npos = exe::training_manager_find_position_training(this, p, &mut q);
    if *mark & 0x10 != 0 && npos == 0 {
        *mark &= 0xef;
    }
    for k in 0..5usize {
        // simplification: the original pushes the category in a dword whose upper bytes are stack leftovers; 0x8b4f50 and
        // 0x8b41d0 leem o byte
        let lv = (exe::training_schedule_get_category_level(prog, k as u32) as u32) as u8 as i8;
        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: the level weight
        let club = training_manager_club_of_contract(training_manager_get_contract_stored(p));
        let fa = exe::training_manager_get_coaching_factor(this, club, k as u32) as u16 as i16 as i32;
        let x = base.wrapping_add(((fa - 100) / 5) as i16);
        let weight: i16 = if x < 0 { 0 } else if x > 200 { 200 } else { x };
        let floor = ((p.professionalism() as i32 / 3) as i8).wrapping_sub(6);
        let ceiling = (p.determination() as i32 / 3) as i8;
        let middle = training_nibble(e, k);
        for _ in 0..weeks {
            let v = middle.wrapping_add(d[k]);
            if p.player().goalkeeper() > 0x12 {
                let r = if k == 4 { friendly_random(2) } else { friendly_random(4) };
                if r != 0 {
                    continue; // goalkeeper: the category barely moves
                }
            }
            if weight > 0x64 {
                if friendly_random(0xd2 - weight as i32) < 0x14 && v < ceiling { // cqd8: the rise random draw
                    if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                        covered!("cqd_8 progress goes up");
                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe
                    }
                    continue;
                }
            }
            if weight >= 0x6e {
                continue;
            }
            if friendly_random(weight as i32 + 0x3c) >= 0x14 || v <= -7 || v <= floor {
                continue;
            }
            if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                covered!("cqd_8 progress goes down");
                d[k] = d[k].wrapping_sub(1); // cqd8: o progresso desce
            }
        }
    }
    let mut laps = weeks;
    let mut contract_of_club = 0usize;
    if skip != 0 || laps != 0 {
        if skip != 0 {
            laps = 1;
        }
        let sheet: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contract_of_club = sheet(read_usize(G_CONTRACT_MANAGER), p.addr(), p.club_job().addr() as u32);
    }
    let mut acabou = false;
    if npos > 0 {
        let mut k = 0i8;
        let mut it = q + 7; // the item’s fee (+7); the rating at +6, the mask at +4
        loop {
            for _ in 0..laps {
                let taxa = read_i8(it);
                if (skip != 0 && read_u16(it - 3) == skip) || taxa > 0 {
                    if friendly_random(read_i8(it) as i32) < 5 {
                        let n = read_i8(it - 1).wrapping_add(2);
                        write_i8(it - 1, if n > 0x14 { 0x14 } else { n });
                    }
                    let c = contract_of_club;
                    if c != 0 && read_usize(c + contract::GRIEVANCES) & (grievances::UNHAPPY_WITH_TRAINING as usize) == 0 {
                        // simplification: the original pushes the mask in ecx with the leftover upper 16 bits; 0x8b4770
                        // reads the word
                        let apt = (exe::training_manager_get_position_aptitude(this, p, read_u16(it - 3) as u32) as u32) as u8 as i8;
                        let t = read_i8(it);
                        let quer = (t > 0x32 && friendly_random(100 - t as i32) < 10) || (apt < 0xc && friendly_random(apt as i32) < 2);
                        let a = p.professionalism();
                        if quer && a < 0xf && friendly_random(a as i32) < 10 && read_usize(G_INJURY) != 0
                            && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 0 {
                            covered!("cqd_8 complains about position training");
                            exe::staff_contract_apply_event_flags(c, 0x8000, 0);
                            let m = (c + contract::MORALE) as *mut i8;
                            let v = *m as i32 - 5;
                            *m = if v > 100 { 100 } else if v < -100 { -100 } else { (*m).wrapping_sub(5) };
                            exe::news_post_player_unhappy_with_training(p);
                            contract_of_club = 0;
                        }
                    }
                } else if taxa < 0 && friendly_random((taxa as i32).abs()) < 5 {
                    let n = read_i8(it - 1).wrapping_sub(2);
                    write_i8(it - 1, n);
                    if n < 1 {
                        write_i8(it, 0);
                        write_i8(it - 1, 1);
                        acabou = true;
                    }
                }
            }
            k = k.wrapping_add(1);
            it += 8;
            if k as i32 >= npos {
                break;
            }
        }
    }
    exe::training_manager_apply_training_progress(this, p, d.as_ptr());
    if acabou {
        covered!("cqd_8 position training ended");
        let n0 = read_i32(this + training_manager_fields::POSITION_TRAINING_COUNT);
        let mut i = 0i32;
        if n0 > 0 {
            loop {
                let base = read_usize(this + training_manager_fields::POSITION_TRAINING_ITEMS);
                let n = read_i32(this + training_manager_fields::POSITION_TRAINING_COUNT);
                let item = base.wrapping_add((i as isize * 8) as usize);
                if read_u8(item + 7) == 0 {
                    let last = base.wrapping_add((n as isize * 8 - 8) as usize);
                    write_u32(item, read_usize(last) as u32);
                    write_u32(item + 4, read_usize(last + 4) as u32);
                    write_i32(this + training_manager_fields::POSITION_TRAINING_COUNT, read_i32(this + training_manager_fields::POSITION_TRAINING_COUNT) - 1);
                }
                i += 1;
                if i >= read_i32(this + training_manager_fields::POSITION_TRAINING_COUNT) {
                    break;
                }
            }
        }
        if n0 != read_i32(this + training_manager_fields::POSITION_TRAINING_COUNT) {
            exe::crt_qsort(read_usize(this + training_manager_fields::POSITION_TRAINING_ITEMS), read_usize(this + training_manager_fields::POSITION_TRAINING_COUNT) as u32, 8, 0x8b4e00);
        }
    }
}

/// `TRAINING_MANAGER::get_player_schedule(p)`, called by the people's daily update (game_daily_staff_update 0x5ce2d0),
/// by `game_check_players_missing_training` and by update_player_training (`update_player_training`): returns the
/// player's training schedule.
///
/// A null person or one without the sheet: the "Error" box (line 0x6ad) and 0. At a human manager's club (contract
/// `exe::contract_manager_get_contract`, kept in [0xae1080]; manager `club::MANAGER` among the last 16 ids): injured
/// (`exe::injury_get_player_injury_status` = 2), the human's schedule 1, this + 0x4d8 + 0x60e × (`G_STAFF_COUNT` − id −
/// 1); otherwise the one of the training entry's slot (+0, outside 0..9 slot 1); outside the 16, the box (line 0x68f)
/// and 0. Otherwise: injured, schedule 0 (this); otherwise this + 0x9b × slot (signed).
/// Original: `TRAINING_MANAGER::get_player_schedule` (0x8b39f0, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_player_schedule(this: usize, p: usize) -> usize {
    let p = Staff(p);
    if p.is_null() || p.player().is_null() {
        show_error_at(0xa8ccd0, 0x6ad);
        return 0;
    }
    let c = training_manager_get_contract_stored(p);
    if c != 0 && training_manager_club_of_contract(c) != 0 {
        let manager = read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + club::MANAGER);
        if manager != 0 && is_human_manager_id(manager) {
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
                if !(0..0x10).contains(&k) {
                    show_error_at(0xa8ccd0, 0x68f);
                    return 0;
                }
                return this.wrapping_add(0x4d8 + 0x60e * k as usize);
            }
            return human_training_schedule(this, manager, read_i8(training_entry_of_staff(this, p)));
        }
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
        return this;
    }
    this.wrapping_add((read_i8(training_entry_of_staff(this, p)) as isize * 0x9b) as usize)
}

/// `TRAINING_SCHEDULE::get_category_level(category)`: returns the level of the schedule's category (+4 + category,
/// signed), in al; the upper bytes of eax keep the sign-extended category (movsx), as in the original. Called by
/// update_player_training (`update_player_training`, the five categories of each update).
/// Original: `TRAINING_SCHEDULE::get_category_level` (0x8b4f50, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_category_level(this: usize, category: u32) -> u32 {
    let k = category as u8 as i8 as i32;
    let v = *((this as isize + k as isize + 4) as *const u8);
    (k as u32 & 0xffff_ff00) | v as u32
}

/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
pub(crate) unsafe fn player_training_entry(this: usize, p: Staff) -> usize {
    read_usize(this + training_manager_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as isize * 6) as usize)
}

/// Signed nibble of category k in a training entry (low/high of +3, +4, low of +5).
pub(crate) unsafe fn training_nibble(e: usize, k: usize) -> i8 {
    let b = read_u8(e + 3 + k / 2);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

/// TRAINING_MANAGER apply_training_progress(person, deltas).
pub(crate) unsafe fn training_manager_apply_training_progress(this: usize, p: Staff, d: &[i8; 8]) {
    exe::training_manager_apply_training_progress(this, p, d.as_ptr());
}

/// `TRAINING_MANAGER::remove_or_restore_training_progress(p, devolver)`: takes the progress stored in the training
/// entry off the attributes or puts it back, around the player attribute normalisations
/// (`staff_normalise_player_attributes` and `staff_normalise_player_physical_attributes`: on ageing in the people's
/// daily update 0x5ce2d0, in the young regen `regenerate_as_youth_player` and on load), which take the progress off
/// before and put it back after.
///
/// A null person or one without the sheet: the "Error" box (line 0x621). Taking off (`devolver` = 0): keeps the five
/// progress nibbles of the training entry in `G_TRAINING_PROGRESS_SAVED_DELTA_0`..`G_TRAINING_PROGRESS_SAVED_DELTA_4`
/// (0xde1f3c..0xde1f40: category 3, 2, 4, 0, 1) and applies the negated progress
/// (`training_manager_apply_training_progress`); restoring: applies the kept one.
/// Original: `TRAINING_MANAGER::remove_or_restore_training_progress` (0x8b3640, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    let p = Staff(p);
    if p.is_null() || p.player().is_null() {
        show_error_at(0xa8ccd0, 0x621);
        return;
    }
    let e = player_training_entry(this, p);
    let mut d = [0i8; 8];
    let g = |a: usize| read_i8(a);
    if devolver != 0 {
        covered!("cqd_9 give back");
        d[0] = g(G_TRAINING_PROGRESS_SAVED_DELTA_0); // cqd9: returns category 0
        d[1] = g(G_TRAINING_PROGRESS_SAVED_DELTA_1);
        d[2] = g(G_TRAINING_PROGRESS_SAVED_DELTA_2);
        d[3] = g(G_TRAINING_PROGRESS_SAVED_DELTA_3);
        d[4] = g(G_TRAINING_PROGRESS_SAVED_DELTA_4); // cqd9: returns category 4
    } else {
        covered!("cqd_9 remove");
        write_i8(G_TRAINING_PROGRESS_SAVED_DELTA_0, training_nibble(e, 0));
        write_i8(G_TRAINING_PROGRESS_SAVED_DELTA_1, training_nibble(e, 1)); // cqd9: guarda a categoria 1
        write_i8(G_TRAINING_PROGRESS_SAVED_DELTA_2, training_nibble(e, 2));
        write_i8(G_TRAINING_PROGRESS_SAVED_DELTA_3, training_nibble(e, 3));
        write_i8(G_TRAINING_PROGRESS_SAVED_DELTA_4, training_nibble(e, 4));
        for k in 0..5 {
            d[k] = training_nibble(e, k).wrapping_neg();
        }
    }
    training_manager_apply_training_progress(this, p, &d);
}

/// `TRAINING_MANAGER::get_training_attribute_change(p, attribute)`, called by the training screen (0x8b8184): how much
/// the training progress adds to an attribute: the attribute (`exe::staff_get_player_attribute`) minus the same
/// attribute without the training progress (it applies the negated progress, reads, and reapplies the progress), in al.
/// A null person or one without the sheet: the "Error" box (line 0x657) and 0.
/// Original: `TRAINING_MANAGER::get_training_attribute_change` (0x8b37c0, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_training_attribute_change(this: usize, p: usize, attribute: u32) -> u8 {
    let p = Staff(p);
    if p.is_null() || p.player().is_null() {
        show_error_at(0xa8ccd0, 0x657);
        return 0;
    }
    let e = player_training_entry(this, p);
    let before = exe::staff_get_player_attribute(p, attribute) as u8;
    let mut d = [0i8; 8];
    for k in 0..5 {
        d[k] = training_nibble(e, k).wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &d);
    let without = exe::staff_get_player_attribute(p, attribute) as u8;
    let mut v = [0i8; 8];
    for k in 0..5 {
        v[k] = d[k].wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &v);
    before.wrapping_sub(without)
}

/// Position masks accepted by `get_position_aptitude` (others show the error box).
pub(crate) fn is_valid_position_mask(m: i32) -> bool {
    matches!(m, 1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800)
}

/// `TRAINING_MANAGER::get_position_aptitude(p, mask)` (`this` not used), called by update_player_training
/// (`update_player_training`, the position training complaint) and by set_position_training_rate
/// (`exe::training_manager_set_position_training_rate`): the player's aptitude to learn a position or side: the average
/// of the adaptability (`.adaptability()`) and the attributes the position needs (ratings 1 to 20 through
/// `exe::player_attribute_rating` over the sheet `.player()`).
///
/// Goalkeeper (1) `player_sheet::POSITIONING`, `player_sheet::HANDLING` and `player_sheet::ONE_ON_ONES`, over 4;
/// defence (2, 4) `player_sheet::TACKLING`, `player_sheet::POSITIONING` and `player_sheet::MARKING`, over 4; midfield
/// and attack (8, 0x10, 0x20) `player_sheet::PASSING`, `player_sheet::MARKING`, `player_sheet::TECHNIQUE` and
/// `player_sheet::TEAMWORK`, over 5; 0x40 `player_sheet::MOVEMENT`, `player_sheet::FINISHING`,
/// `player_sheet::TECHNIQUE` and `player_sheet::PLAYER_PACE`, over 5; 0x80 `player_sheet::LEFT_FOOT`, over 2; 0x200
/// only the adaptability (in al, the upper bytes with the mask); 0x800 `player_sheet::RIGHT_FOOT`, over 2 (truncated
/// divisions). Any other mask: the "Error" box (line 0x945) and 0.
/// Original: `TRAINING_MANAGER::get_position_aptitude` (0x8b4770, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_position_aptitude(this: usize, p: usize, mask: u32) -> u32 {
    let p = Staff(p);
    let _ = this;
    let m = mask as u16 as i16 as i32;
    if !is_valid_position_mask(m) {
        show_error_at(0xa8ccd0, 0x945);
        return 0;
    }
    let f = p.player();
    // simplification: the original pushes the attribute byte in a register with leftover upper bytes; 0x544380 reads
    // o byte
    let rating = |o: usize| (exe::player_attribute_rating(f, read_u8(f.addr() + o) as u32) as u32) as u8 as i8 as i32;
    let b = |o: usize| read_i8(f.addr() + o) as i32;
    let adapt = p.adaptability() as i32;
    let r = match m {
        1 => {
            let x = rating(player_sheet::POSITIONING);
            let y = rating(player_sheet::HANDLING);
            (b(player_sheet::ONE_ON_ONES) + x + y + adapt) / 4
        }
        2 | 4 => {
            let x = rating(player_sheet::TACKLING);
            let y = rating(player_sheet::POSITIONING);
            let z = rating(player_sheet::MARKING);
            (x + y + z + adapt) / 4
        }
        8 | 0x10 | 0x20 => {
            let x = rating(player_sheet::PASSING);
            let y = rating(player_sheet::MARKING);
            (x + y + adapt + b(player_sheet::TECHNIQUE) + b(player_sheet::TEAMWORK)) / 5
        }
        0x40 => {
            let x = rating(player_sheet::MOVEMENT);
            let y = rating(player_sheet::FINISHING);
            (x + y + adapt + b(player_sheet::TECHNIQUE) + b(player_sheet::PLAYER_PACE)) / 5
        }
        0x80 => (adapt + b(player_sheet::LEFT_FOOT)) / 2,
        0x200 => return (m as u32 & 0xffff_ff00) | (p.adaptability() as u8) as u32,
        _ => (adapt + b(player_sheet::RIGHT_FOOT)) / 2,
    };
    r as u32
}

/// Writes the five levels (+4..+8) of a training schedule.
pub(crate) unsafe fn set_schedule_levels(this: usize, n: [u8; 5]) {
    for (k, v) in n.iter().enumerate() {
        write_u8(this + 4 + k, *v);
    }
}

/// `TRAINING_SCHEDULE::set_type(slot, kind)`: stores the schedule's slot and type and its five category levels from the
/// type's mix.
///
/// The slot (signed, as a dword) at +0, the type at `training_schedule::TYPE` and the five category levels (+4..+8) by
/// the type's mix (jump table 0x8b4f08): 1 general (25 in the first four), 2 to 5 one category at 50 and the others at
/// 10, 6 goalkeepers (50 in the fifth, 0 in the third), 0 and 7 no training (all 0); outside 0..=7 (the −1 of the
/// human's first slot) only the slot and the type. Called by set_default_schedules (`set_default_schedules`, 167 times
/// at the start of the game) and by the training screens.
/// Original: `TRAINING_SCHEDULE::set_type` (0x8b4e50, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_type(this: usize, slot: i8, kind: i8) {
    write_i32(this, slot as i32);
    write_i8(this + training_schedule::TYPE, kind);
    match kind {
        1 => {
            covered!("cpu_8 general");
            set_schedule_levels(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral
        }
        2 => set_schedule_levels(this, [0x32, 0xa, 0xa, 0xa, 0]),
        3 => set_schedule_levels(this, [0xa, 0xa, 0x32, 0xa, 0]),
        4 => set_schedule_levels(this, [0xa, 0xa, 0xa, 0x32, 0]),
        5 => set_schedule_levels(this, [0xa, 0x32, 0xa, 0xa, 0]),
        6 => {
            covered!("cpu_8 goalkeepers");
            set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: the goalkeeper training
        }
        0 | 7 => set_schedule_levels(this, [0; 5]),
        _ => {}
    }
}

/// Training schedule slot of human manager `pessoa` as `set_default_schedules` computes it; outside the 16 humans shows the box and
/// returns 0.
pub(crate) unsafe fn human_schedule_slot(this: usize, person: usize, slot: usize) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(person)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    this + training_manager_fields::HUMAN_SCHEDULES + 0x9b * slot + 0x60e * v as usize
}

/// `TRAINING_MANAGER::set_default_schedules()`, called by the constructor (0x8b0ec0) in a new game (without
/// training.dat): sets up the computer's seven schedules and the ten schedules of each of the 16 human managers, then
/// gives the default names.
///
/// The computer's seven schedules (0x9b bytes each from this; type k in slot k: the computer trains through slot 1,
/// general, and slot 6, goalkeepers, according to the player's training entry) and, for each of the 16 human managers
/// (person `G_STAFF_COUNT` − h − 1, 0x6e-byte record in `G_STAFF_TABLE`), the ten schedules: slots 0 to 5 computed here
/// (types −1, 0, 1, 2, 3, 4), 6 to 9 through `exe::training_manager_get_manager_schedule` (types 5, 6, 7, 7). At the
/// end the default names (`exe::training_manager_set_default_schedule_names`). Called by address
/// (`exe::training_schedule_set_type` and `exe::training_manager_get_manager_schedule` in this batch).
/// Original: `TRAINING_MANAGER::set_default_schedules` (0x8b3d40, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_default_schedules(this: usize) {
    exe::training_schedule_set_type(this, 0, 0);
    covered!("cpu_8 default schedules");
    exe::training_schedule_set_type(this + 0x9b, 1, 1); // cpu8: the general computer programme
    for k in 2..7i8 {
        exe::training_schedule_set_type(this + 0x9b * k as usize, k, k);
    }
    for h in 0..0x10i32 {
        let id = read_i32(G_STAFF_COUNT).wrapping_sub(h).wrapping_sub(1);
        let person = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul((staff::SIZE as u32)) as usize);
        for (vaga, t) in [(0usize, -1i8), (1, 0), (2, 1), (3, 2), (4, 3), (5, 4)] {
            exe::training_schedule_set_type(human_schedule_slot(this, person, vaga), vaga as i8, t);
        }
        // simplification: the original pushes slots 8 and 9 in a single dword with only the low byte written; both calls read the byte.
        for (vaga, t) in [(6i8, 5i8), (7, 6), (8, 7), (9, 7)] {
            exe::training_schedule_set_type(exe::training_manager_get_manager_schedule(this, person, vaga), vaga, t);
        }
    }
    exe::training_manager_set_default_schedule_names(this);
}

/// `TRAINING_MANAGER::get_manager_schedule(manager, slot)`: returns the human manager's schedule for the slot (0..=9;
/// outside that, 1): this + 0x9b × (slot + 10 × human + 7), human = `G_STAFF_COUNT` − id − 1. Outside the 16 human
/// managers: the box (line 0x68f) and 0. Called by `set_default_schedules` (slots 6 to 9) and by the screens.
/// Original: `TRAINING_MANAGER::get_manager_schedule` (0x8b3930, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_manager_schedule(this: usize, manager: usize, slot: i8) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let slot = if (0..0xa).contains(&slot) { slot } else { 1 };
    this.wrapping_add(0x9b * (slot as i32 + 10 * v + 7) as usize)
}

/// Divisor of the coaches' average by number of coaches (1..6): 1, 1.45, 1.7, 1.9, 2.05, 2.2.
pub(crate) fn coach_average_divisor(n: i8) -> f64 {
    const T: [u32; 6] = [0x3f800000, 0x3fb9999a, 0x3fd9999a, 0x3ff33333, 0x40033333, 0x400ccccd];
    f32::from_bits(T[(n - 1) as usize]) as f64
}

/// `TRAINING_MANAGER::get_coaching_factor(club, category)`, called by TRAINING_MANAGER::update_player_training
/// (`update_player_training`, five times per player in each training update; the value goes in as (factor − 100) / 5
/// into the weight of the progress roll): the coaching factor of a training category.
///
/// A null club: 0. Without a manager (`.manager()`) or with a computer manager (id below `G_STAFF_COUNT` − 16): 100
/// (eax carries the upper 16 bits of the manager pointer, as in the original). A human manager: his staff
/// (`exe::training_manager_get_coach_assignments`, six 9-byte slots: id −1 is an empty slot, the category rating at +4
/// + category); with n coaches with a rating, the sum (16 bits) divided by the nth divisor and truncated (_ftol
/// 0x945640), × 40/7 (`rdata::F64_5_714285714`) and truncated; below 50, 50. Hot on the computer branch: no counter
/// there.
/// Original: `TRAINING_MANAGER::get_coaching_factor` (0x8b41d0, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_coaching_factor(this: usize, club: usize, category: i8) -> u32 {
    let club = Club(club);
    let _ = this;
    if club.is_null() {
        return 0;
    }
    let t = club.manager().addr();
    if t == 0 {
        covered!("cpt_8 without manager");
        return 100; // cpt8: no manager
    }
    if read_i32(t) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador
    }
    let s = exe::training_manager_get_coach_assignments(this, t);
    let (mut n, mut soma) = (0i8, 0i16);
    for k in 0..6usize {
        let e = s + 9 * k;
        if read_i32(e) == -1 {
            continue;
        }
        let rating = *((e as isize + 4 + category as isize) as *const i8);
        if rating != 0 {
            n += 1;
            soma = soma.wrapping_add(rating as i16);
        }
    }
    if n != 0 {
        covered!("cpt_8 coaches");
        soma = ftol(soma as f64 / coach_average_divisor(n)) as i16; // cpt8: the average of the coaches
    }
    let r = ftol(soma as f64 * read_f64(rdata::F64_5_714285714));
    if (r as i16) < 0x32 {
        return 0x32;
    }
    r as u32
}

/// `TRAINING_MANAGER::get_coach_assignments(manager)`: returns the training staff table (six 9-byte slots) of one of
/// the 16 human managers, at this + `training_manager_fields::COACH_ASSIGNMENTS` + 54 × (`G_STAFF_COUNT` − id − 1).
///
/// A null manager: the "Error" box (line 0xa20) and 0; outside the 16 human managers: the box (line 0xa2a) and 0.
/// Original: `TRAINING_MANAGER::get_coach_assignments` (0x8b4cd0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_coach_assignments(this: usize, manager: usize) -> usize {
    if manager == 0 {
        show_error_at(0xa8ccd0, 0xa20);
        return 0;
    }
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0xa2a);
        return 0;
    }
    this + training_manager_fields::COACH_ASSIGNMENTS + 54 * v as usize
}

/// `TRAINING_SCHEDULE::get_training_load()`: the schedule's load from its five category levels (+4..+8, signed) with
/// fixed weights, up to 100.
///
/// trunc(+8 × 0.5) − trunc(+4 × −1.3) − trunc(+7 × −0.5) − trunc(+6 × −0.5) − trunc(+5 × −0.5) (constants
/// `rdata::F64_0_5`, `rdata::F64_NEG_1_3`, `rdata::F64_NEG_0_5`; _ftol 0x945640), in 16 bits; from 100 up, 100. Called
/// by update_player_training (`update_player_training`, the human club's player), by set_player_schedule (0x8b2250) and
/// by reassign_squad_schedules (`reassign_squad_schedules`); it goes to the fitness sheet (Injury::set_training_load
/// `set_training_load`).
/// Original: `TRAINING_SCHEDULE::get_training_load` (0x8b4f60, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_training_load(this: usize) -> i32 {
    let a = ftol((read_i8(this + 4) as f64) * read_f64(rdata::F64_NEG_1_3)) as i16;
    let mut s = ftol((read_i8(this + 8) as f64) * read_f64(rdata::F64_0_5)) as i16;
    s = s.wrapping_sub(a);
    s = s.wrapping_sub(ftol((read_i8(this + 7) as f64) * read_f64(rdata::F64_NEG_0_5)) as i16);
    s = s.wrapping_sub(ftol((read_i8(this + 6) as f64) * read_f64(rdata::F64_NEG_0_5)) as i16);
    s = s.wrapping_sub(ftol((read_i8(this + 5) as f64) * read_f64(rdata::F64_NEG_0_5)) as i16);
    if s >= 0x64 {
        return 0x64;
    }
    s as i32
}

/// Adds v to the 1..20 attribute byte at a, clamped to 1..20.
pub(crate) unsafe fn add_clamped_1_20(a: usize, v: i32) {
    write_i8(a, (read_i8(a) as i32 + v).clamp(1, 20) as i8);
}

/// Adds v to the -125..125 attribute byte at a, clamped.
pub(crate) unsafe fn add_clamped_125(a: usize, v: i32) {
    write_i8(a, (read_i8(a) as i32 + v).clamp(-125, 125) as i8);
}

/// How much the attribute moves for progress d over the stored nibble n: (d + n)/2 - n/2 in 8 bits.
pub(crate) fn progress_step(d: i8, n: i32) -> i32 {
    (((d as i32 + n) / 2) as i8).wrapping_sub((n / 2) as i8) as i32
}

/// Adds d to the high nibble of the byte at a (low kept).
pub(crate) unsafe fn add_high_nibble(a: usize, d: i8) {
    let v = read_u8(a);
    write_u8(a, ((v & 0xf0).wrapping_add((d as u8) << 4)) | (v & 0x0f));
}

/// Adds d to the low nibble of the byte at a (high kept).
pub(crate) unsafe fn add_low_nibble(a: usize, d: i8) {
    let v = read_u8(a);
    write_u8(a, (v & 0xf0) | (v.wrapping_add(d as u8) & 0x0f));
}

/// `TRAINING_MANAGER::apply_training_progress(person, d)`, called by update_player_training (`update_player_training`,
/// each player on each training update) and by `remove_or_restore_training_progress` and
/// `get_training_attribute_change`: adds each category's progress to the player's stored nibble, moving the category
/// attributes, then handles the trained position or side.
///
/// A null person: the "Error" box (line 0x4b3); without the sheet (`.player()`): the box (line 0x4ba); an id from
/// `G_STAFF_COUNT` on: nothing. With the player's 6-byte entry (this `training_manager_fields::PLAYER_ENTRIES` + 6 ×
/// sheet id), each category with progress `d[k]` other than zero moves its step (`progress_step` over the stored
/// nibble) on its sheet attributes (those from 1 to 20 by the step, those from −125 to 125 by six times the step; every
/// two steps the attributes move) and stores the nibble plus `d[k]`: 3 (+4 high), 0 (+3 low), 2 (+4 low), 1 (+3 high)
/// and 4 (+5 low).
///
/// Then the position training (`exe::training_manager_find_position_training`): with the learning entry at 15 or more,
/// the position or side of the mask goes to 20 (learnt); below that, it goes back to 1 (training stopped) if more than
/// one position (`player_sheet::GOALKEEPER`..`player_sheet::ATTACKER`) or side (`player_sheet::RIGHT_SIDE`,
/// `player_sheet::LEFT_SIDE`, `player_sheet::CENTRAL`) at 15 or more is still left. Hot: no counter.
/// Original: `TRAINING_MANAGER::apply_training_progress` (0x8b2be0, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn apply_training_progress(this: usize, person: usize, d: usize) {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa8ccd0, 0x4b3);
        return;
    }
    let f = person.player();
    if f.is_null() {
        show_error_at(0xa8ccd0, 0x4ba);
        return;
    }
    if read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let e = read_usize(this + training_manager_fields::PLAYER_ENTRIES) + 6 * read_i32(f.addr()) as usize;
    // category 3 (the high nibble of +4)
    let k = read_i8(d + 3);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) >> 4) as i32);
        if dd != 0 {
            let six = dd * 6;
            add_clamped_1_20(f.addr() + player_sheet::CORNERS, dd);
            add_clamped_125(f.addr() + player_sheet::CROSSING, six);
            add_clamped_125(f.addr() + player_sheet::DRIBBLING, six);
            add_clamped_1_20(f.addr() + player_sheet::FREE_KICKS, dd);
            add_clamped_125(f.addr() + player_sheet::HEADING, six);
            add_clamped_125(f.addr() + player_sheet::PASSING, six);
            add_clamped_125(f.addr() + player_sheet::TACKLING, six);
            add_clamped_1_20(f.addr() + player_sheet::TECHNIQUE, dd);
            add_clamped_125(f.addr() + player_sheet::THROW_INS, six);
        }
        add_high_nibble(e + 4, k); // cpt9: stores the progress of category 3
    }
    // category 0 (the low nibble of +3)
    let k = read_i8(d);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) << 4 >> 4) as i32);
        if dd != 0 {
            add_clamped_1_20(f.addr() + player_sheet::ACCELERATION, dd); // cpt9: the first attribute of category 0
            add_clamped_1_20(f.addr() + player_sheet::AGILITY, dd);
            add_clamped_1_20(f.addr() + player_sheet::JUMPING, dd);
            add_clamped_1_20(f.addr() + player_sheet::NATURAL_FITNESS, dd);
            add_clamped_1_20(f.addr() + player_sheet::PLAYER_PACE, dd);
            add_clamped_125(f.addr() + player_sheet::REFLEXES, dd * 6);
            add_clamped_1_20(f.addr() + player_sheet::STAMINA, dd);
            add_clamped_1_20(f.addr() + player_sheet::STRENGTH, dd);
        }
        add_low_nibble(e + 3, k);
    }
    // category 2 (the low nibble of +4)
    let k = read_i8(d + 2);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) << 4 >> 4) as i32);
        if dd != 0 {
            let six = dd * 6;
            add_clamped_125(f.addr() + player_sheet::FINISHING, six);
            add_clamped_125(f.addr() + player_sheet::LONG_SHOTS, six);
            add_clamped_125(f.addr() + player_sheet::PENALTIES, six);
        }
        add_low_nibble(e + 4, k);
    }
    // category 1 (the high nibble of +3)
    let k = read_i8(d + 1);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) >> 4) as i32);
        if dd != 0 {
            let six = dd * 6; // cpt9: category 1, six times the step
            add_clamped_125(f.addr() + player_sheet::DECISIONS, six);
            add_clamped_125(f.addr() + player_sheet::MARKING, six);
            add_clamped_125(f.addr() + player_sheet::MOVEMENT, six);
            add_clamped_1_20(f.addr() + player_sheet::TEAMWORK, dd);
            add_clamped_125(f.addr() + player_sheet::POSITIONING, six);
        }
        add_high_nibble(e + 3, k);
    }
    // category 4 (the low nibble of +5)
    let k = read_i8(d + 4);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 5) << 4 >> 4) as i32);
        if dd != 0 {
            let six = dd * 6;
            add_clamped_125(f.addr() + player_sheet::HANDLING, six);
            add_clamped_125(f.addr() + player_sheet::ONE_ON_ONES, six);
        }
        add_low_nibble(e + 5, k);
    }
    // position training
    let mut p = 0usize;
    let n = exe::training_manager_find_position_training(this, person, &mut p);
    if n <= 0 {
        return;
    }
    let de15 = |o: usize| (read_i8(f.addr() + o) >= 0xf) as i8;
    let mut positions = de15(player_sheet::GOALKEEPER) + de15(player_sheet::SWEEPER) + de15(player_sheet::DEFENDER) + de15(player_sheet::DEFENSIVE_MIDFIELDER) + de15(player_sheet::MIDFIELDER) + de15(player_sheet::ATTACKING_MIDFIELDER) + de15(player_sheet::ATTACKER);
    let mut sides = de15(player_sheet::LEFT_SIDE) + de15(player_sheet::CENTRAL) + de15(player_sheet::RIGHT_SIDE);
    for j in 0..n as usize {
        let item = p + 8 * j;
        let field = match read_i16(item + 4) as i32 {
            1 => 0xf,
            2 => 0x10,
            4 => 0x11,
            8 => 0x12,
            0x10 => 0x13,
            0x20 => 0x14,
            0x40 => 0x15,
            0x80 => 0x18,
            0x200 => 0x19,
            0x800 => 0x17,
            _ => continue,
        };
        if read_i8(item + 6) >= 0xf {
            write_u8(f.addr() + field, 0x14);
        } else if field == 0x18 || field == 0x19 || field == 0x17 {
            if sides > 1 {
                write_u8(f.addr() + field, 1);
                sides -= 1;
            }
        } else if positions > 1 {
            write_u8(f.addr() + field, 1);
            positions -= 1;
        }
    }
}

/// `TRAINING_MANAGER::find_position_training(person, out)`: finds the player's first entry in the sorted position
/// training list and returns how many are his.
///
/// A null person, the "Error" box (line 0x995); a null output, the box (line 0x99c); without the sheet (`.player()`),
/// the box (line 0x9a3): 0. Without flag 0x10 in +5 of the player's training entry (this
/// `training_manager_fields::PLAYER_ENTRIES` + 6 × sheet id), the position training mark: 0. Otherwise the binary
/// search (`FN_CRT_BSEARCH`, order 0x8b4e00) of the person's id in the position training list
/// (`training_manager_fields::POSITION_TRAINING_ITEMS`, count `training_manager_fields::POSITION_TRAINING_COUNT`,
/// 8-byte items); not found: 0; found: steps back to the person's first item, stores it in the output and returns how
/// many consecutive ones are hers.
/// Original: `TRAINING_MANAGER::find_position_training` (0x8b4b20, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_position_training(this: usize, person: usize, out: *mut usize) -> i32 {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa8ccd0, 0x995);
        return 0;
    }
    if out.is_null() {
        show_error_at(0xa8ccd0, 0x99c);
        return 0;
    }
    let f = person.player();
    if f.is_null() {
        show_error_at(0xa8ccd0, 0x9a3);
        return 0;
    }
    if read_u8(read_usize(this + training_manager_fields::PLAYER_ENTRIES) + 6 * read_i32(f.addr()) as usize + 5) & 0x10 == 0 {
        return 0;
    }
    let key = read_i32(person.addr());
    let search: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let base = read_usize(this + training_manager_fields::POSITION_TRAINING_ITEMS);
    let mut a = search(&key, base, read_usize(this + training_manager_fields::POSITION_TRAINING_COUNT), 8, 0x8b4e00);
    if a == 0 {
        return 0;
    }
    while a > base {
        if read_i32(a - 8) != key {
            break;
        }
        a -= 8;
    }
    *out = a;
    let last = read_usize(this + training_manager_fields::POSITION_TRAINING_ITEMS) + read_usize(this + training_manager_fields::POSITION_TRAINING_COUNT) * 8 - 8;
    let mut n = 0i32;
    while a <= last && read_i32(a) == key {
        a += 8;
        n += 1;
    }
    n
}

/// `TRAINING_MANAGER::has_fast_position_training(person)`, called by the people's daily update (game_daily_staff_update
/// 0x5ce2d0): returns 1 if some item of the person's position training (`exe::training_manager_find_position_training`)
/// has a learning rate (+7) above 50; otherwise 0.
/// Original: `TRAINING_MANAGER::has_fast_position_training` (0x8b4db0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn has_fast_position_training(this: usize, person: usize) -> u32 {
    let mut p = 0usize;
    let n = exe::training_manager_find_position_training(this, person, &mut p);
    for j in 0..n.max(0) as usize {
        if read_i8(p + 8 * j + 7) > 0x32 {
            return 1;
        }
    }
    0
}

/// `TRAINING_MANAGER::advance_week_counter(&day)`, every day from the day loop (game_main_day_loop 0x5cccd0, with the
/// day of the year of the game date 0xae2c90): on a day that is a multiple of 7 (signed remainder), the training week
/// (`training_manager_fields::WEEK_COUNTER`, 16 bits) goes up by 1; update_player_training (`update_player_training`)
/// catches up each player with the missing weeks. Returns day / 7 (the caller does not read it).
/// Original: `TRAINING_MANAGER::advance_week_counter` (0x8b1730, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn advance_week_counter(this: usize, day: *const i16) -> i32 {
    let v = *day as i32;
    let rest = v % 7; // cpt10: the 7-day week
    if rest == 0 { // cpt10: the seventh day
        covered!("cpt_10 week");
        let s = (this + training_manager_fields::WEEK_COUNTER) as *mut u16;
        *s = (*s).wrapping_add(1); // cpt10: one more week
    }
    v / 7
}

// simplification: the reserve loses the item (does not run twice): original only; the copy takes the branch.

/// `TRAINING_MANAGER::init_player_entries()`, called by the constructor (0x8b0ec0) in a new game: allocates the 6-byte
/// training entry of every player sheet and puts goalkeepers on schedule 6 and the others on 1.
///
/// Allocates (calloc `exe::crt_calloc`) the 6-byte training entries, one per player sheet (`G_PLAYER_DATA_COUNT` sheets
/// of 0x46 bytes in `G_PLAYER_DATA_TABLE`, the sheet by its id +0), at this +
/// `training_manager_fields::PLAYER_ENTRIES`; without memory, the box (line 0x6e3) and it returns. In each entry it
/// clears bit 0x20 of +5 and sets the schedule slot (+0): 6 (goalkeepers) with a goalkeeping rating
/// (`player_sheet::GOALKEEPER`, signed) of 15 or more, otherwise 1 (general). Each player's first training update
/// (`update_player_training`, which only afterwards redoes the slot of the computer's players) and all of the human
/// club's training use this slot.
/// Original: `TRAINING_MANAGER::init_player_entries` (0x8b3c50, training_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn init_player_entries(this: usize) {
    let p = exe::crt_calloc(read_usize(G_PLAYER_DATA_COUNT) as u32, 6);
    write_u32(this + training_manager_fields::PLAYER_ENTRIES, p as u32);
    if p == 0 {
        show_error_at(0xa8ccd0, 0x6e3);
        return;
    }
    let mut f = read_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        let sheet = || read_usize(this + training_manager_fields::PLAYER_ENTRIES).wrapping_add((read_i32(f) as usize).wrapping_mul(6));
        *((sheet() + 5) as *mut u8) &= 0xdf;
        let goal = read_i8(f + player_sheet::GOALKEEPER);
        if goal >= 0xf { // cpu9: goalkeeper from 15 up
            covered!("cpu_9 goalkeeper");
            write_u8(sheet(), 6); // cpu9: the goalkeepers programme
        } else {
            covered!("cpu_9 outfield");
            write_u8(sheet(), 1); // cpu9: o programa geral
        }
        i += 1;
        f += 0x46;
    }
}

