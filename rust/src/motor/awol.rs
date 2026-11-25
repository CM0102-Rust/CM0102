use super::*;

/// The items (node +0 pointers) of an LLIST, without touching its cursor.
pub(crate) unsafe fn llist_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut in_the = read_usize(list + 2);
    while in_the != 0 && v.len() < 0x10000 {
        v.push(read_usize(in_the));
        in_the = read_usize(in_the + 6);
    }
    v
}

/// Shared body of { `clear_awol` } and { `clear_missed_training` }: removes the person's first entry from a list when the person has the
/// mark bit.
///
/// With the person's mark `bit` ([this+0xc] + id), the mark is flipped (the original's `or bit` never yields zero) and,
/// with the list ([this + `lista`]), the cursor goes back to the head and, among the first `conta` (+0, signed word)
/// items, the first one with the person's id at +0 is removed (`manager_manager_remove`, which frees the node). A list that ends before
/// the count is reached stops the loop.
pub(crate) unsafe fn toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize, foul: bool) {
    if person == 0 {
        return;
    }
    let marks = read_usize(this + awol_fields::PERSON_FLAGS);
    if marks == 0 {
        return;
    }
    let p = marks.wrapping_add(read_usize(person));
    let m = read_u8(p);
    if m & bit == 0 {
        return;
    }
    if foul {
        covered!("cqd_11 absence removed");
    }
    write_u8(p, m ^ bit);
    let l = read_usize(this + list);
    if l == 0 {
        return;
    }
    write_usize(l + llist::CURSOR, read_usize(l + 2));
    // 0x68d1b0: mov ecx, [edi + lista] -> thiscall; 0x68d1f0: push item; mov ecx, [edi + lista]; ret 4 -> thiscall
    let mut i: i32 = 0;
    if read_i16(l) as i32 <= 0 {
        return;
    }
    loop {
        let item = exe::llist_get_next(read_usize(this + list));
        if item == 0 {
            return;
        }
        if read_usize(item) == read_usize(person) {
            if foul {
                covered!("cqd_11 absence record removed");
            }
            exe::llist_remove(read_usize(this + list), item);
            return;
        }
        i += 1;
        if i >= read_i16(read_usize(this + list)) as i32 {
            return;
        }
    }
}

/// `AWOL::clear_awol(person)`: ends the absence. Called by the end of the absence without a contract (`is_player_awol`), by
/// retirements (`process_retirements`) and by transfers (`free_transfer_staff`, `handle_player_accepted_terms`, `exe::transfer_manager_move_player_to_club`).
///
/// With the person and the flag table ([this+0xc]) not null and bit 1 of the person's flag set: flips the bit and
/// removes the person's record from the absence list ([this]).
/// Original: `AWOL::clear_awol` (0x4196a0, awol.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_awol(this: usize, person: usize) {
    toggle_mark_and_unlist(this, person, 1, 0, false);
}

/// `AWOL::clear_missed_training(person)`: ends the missed training. Called by the missed-training check for someone
/// left without a contract (`has_missed_training`, which the daily player update consults) and by retirements (`process_retirements`).
///
/// With the person and the flag table not null and bit 2 of the flag set: flips the bit and removes the person's record
/// from the missed training list ([this+4]).
/// Original: `AWOL::clear_missed_training` (0x4197c0, awol.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_missed_training(this: usize, person: usize) {
    toggle_mark_and_unlist(this, person, 2, 4, true);
}

/// Plans toggle_mark_and_unlist: the mark value after and the list items that remain.
pub(crate) unsafe fn plan_toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize) -> (Option<(usize, u8)>, usize, Vec<usize>) {
    let l = read_usize(this + list);
    let items = llist_items(l);
    if person == 0 || read_usize(this + awol_fields::PERSON_FLAGS) == 0 {
        return (None, l, items);
    }
    let p = read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(read_usize(person));
    let m = read_u8(p);
    if m & bit == 0 {
        return (Some((p, m)), l, items);
    }
    if l == 0 {
        return (Some((p, m ^ bit)), l, items);
    }
    let count = (read_i16(l) as i32).max(0) as usize;
    let mut stays = items.clone();
    if let Some(k) = items.iter().take(count).position(|&x| read_usize(x) == read_usize(person)) {
        stays.remove(k);
    }
    (Some((p, m ^ bit)), l, stays)
}

/// Comparer of the two: runs only the original and checks the mark and remaining list against the plan.
pub(crate) unsafe fn compare_toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize, o: unsafe extern "thiscall" fn(usize, usize), name: &str) {
    let (marca, l, fica) = plan_toggle_mark_and_unlist(this, person, bit, list);
    o(this, person);
    let after = llist_items(l);
    let mark_ok = marca.map_or(true, |(p, m)| read_u8(p) == m);
    SCOREBOARD.record(mark_ok && after == fica, || format!("{}(person {:#x}) mark plan {:x?} now {:x?}; list plan {:x?} now {:x?}",
        name, person, marca, marca.map(|(p, _)| read_u8(p)), fica, after));
}

/// The rehearsal of the news item. The random draws come from the original routine's tape (n and the answer) and,
/// instead of delivering the news, the copy records the destination (1 the player's club, 2 the club's nation, 0 none).
/// The tape keeps the next position and whether any n did not match.
pub(crate) static mut AWOL_NEWS_TAPE: Option<(Vec<(i32, i32)>, usize, bool)> = None;
pub(crate) static mut AWOL_NEWS_DESTINATION: (u32, usize) = (0, 0);

/// Random draw: replays from AWOL_NEWS_TAPE when set (marking failure on mismatch), else the game's random.
pub(crate) unsafe fn taped_random(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE);
    match f {
        Some((v, k, falhou)) => match v.get(*k) {
            Some(&(m, r)) if m == n && !*falhou => {
                *k += 1;
                r
            }
            _ => {
                *falhou = true;
                0
            }
        },
        None => {
            exe::utils_random(n)
        }
    }
}

/// `awol_post_missed_training_news(player, club)`: posts the 0xbc9 missed training news. Called by the missed training
/// (`miss_training`). SEH frame for the local news, no destructor.
///
/// With a player who has a player sheet (`.player()`) and a club with a manager (`club::MANAGER`): the news 0xbc9
/// (`exe::new_newsitem`) with the player (id and the `names::ID` keys of `first_name()`, `second_name()`,
/// `common_name()`), the club id, draw(3), whether the club is human-run (`exe::human_manager_is_human_club` with 0, 1)
/// and who reports it: the manager, if the club has a manager and is not human-run (`exe::human_manager_is_human_club`
/// with 1, 1; id and keys); otherwise, with draw(2), one of the 5 coaches (`club::COACHES`, draw(5) until one is found
/// or a thousand tries), and without the draw the assistant (`club::ASSISTANT_MANAGER`); with nobody (or id −1), −1 in
/// all four.
///
/// Human-run club (`exe::news_club_has_human_manager`): sent to the player's club (`club_job()`,
/// `exe::news_send_to_club_manager`); otherwise, if the club has a nation (`club::NATION`) and the player's reputation
/// (`.player().current_reputation()`) is above 7250, to the nation's inboxes (`exe::news_send_to_nation_managers`).
/// Original: `awol_post_missed_training_news` (0x41a630, awol.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn awol_post_missed_training_news(this: usize, player: usize, club: usize) {
    let player = Staff(player);
    let _ = this;
    if player.is_null() || player.player().is_null() || club == 0 || read_usize(club + club::MANAGER) == 0 { // cqd11: no player or manager
        return;
    }
    // 0x77bbe0: push 0; push 0xbc9; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall; 0x600a50: 5
    // pushes; add esp, 0x14 -> cdecl (club, 0, 1, 0, 0) and (club, 1, 1, 0, 0)
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbc9, 0);
    exe::newsitem_set_slot(nt, 0, read_usize(player.addr()) as u32);
    exe::newsitem_set_slot(nt, 1, read_usize(player.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 2, read_usize(player.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 3, read_usize(player.common_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 4, read_usize(club) as u32);
    covered!("cqd_11 absence news");
    let r = taped_random(3); // cqd11: the news random draw
    exe::newsitem_set_slot(nt, 5, r as u32);
    exe::newsitem_set_slot(nt, 6, exe::human_manager_is_human_club(club, 0, 1, 0, 0));
    let manager = read_usize(club + club::MANAGER);
    if manager != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 { // cqd11: the AI manager reports
        covered!("cqd_11 technical reporter");
        let t = Staff(read_usize(club + club::MANAGER));
        exe::newsitem_set_slot(nt, 7, read_usize(t.addr()) as u32);
        exe::newsitem_set_slot(nt, 8, read_usize(t.first_name() + names::ID) as u32);
        exe::newsitem_set_slot(nt, 9, read_usize(t.second_name() + names::ID) as u32);
        exe::newsitem_set_slot(nt, 0xa, read_usize(t.common_name() + names::ID) as u32);
    } else {
        let who = if taped_random(2) != 0 {
            let mut k;
            let mut laps = 0;
            loop {
                k = taped_random(5);
                laps += 1;
                if laps >= 1000 || read_usize(club.wrapping_add(k as usize * 4) + club::COACHES) != 0 {
                    break;
                }
            }
            read_usize(club.wrapping_add(k as usize * 4) + club::COACHES)
        } else {
            read_usize(club + club::ASSISTANT_MANAGER)
        };
        let id = if who != 0 { read_usize(who) as i32 } else { -1 };
        if id == -1 {
            for k in 7..=0xa {
                exe::newsitem_set_slot(nt, k, u32::MAX);
            }
        } else {
            let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
            exe::newsitem_set_slot(nt, 7, id as u32);
            exe::newsitem_set_slot(nt, 8, read_usize(p.first_name() + names::ID) as u32);
            exe::newsitem_set_slot(nt, 9, read_usize(p.second_name() + names::ID) as u32);
            exe::newsitem_set_slot(nt, 0xa, read_usize(p.common_name() + names::ID) as u32);
        }
    }
    let news = read_usize(G_NEWS);
    // 0x7875e0: mov ecx, [0xb7431c]; push club (ret 4); 0x7861e0: push club +0x39; push news; mov ecx, [0xb7431c];
    // 0x7863f0: push nation; push news; mov ecx, [0xb7431c] -> thiscall in all three
    let dry_run = (*core::ptr::addr_of!(AWOL_NEWS_TAPE)).is_some();
    if exe::news_club_has_human_manager(news, club) != 0 {
        if dry_run {
            AWOL_NEWS_DESTINATION = (1, player.club_job().addr());
        } else {
            exe::news_send_to_club_manager(news, nt, player.club_job());
        }
        return;
    }
    let nation = read_usize(club + club::NATION);
    if nation == 0 || player.player().current_reputation() <= 0x1c52 {
        return;
    }
    if dry_run {
        AWOL_NEWS_DESTINATION = (2, nation);
    } else {
        exe::news_send_to_nation_managers(news, nt, nation);
    }
}

/// Decrements a signed attribute byte (above 1) depending on whether random(attribute) is zero.
pub(crate) unsafe fn maybe_drop_attribute(a: usize, zero: bool) {
    let v = read_u8(a) as i8;
    if v > 1 && (friendly_random(v as i32) == 0) == zero {
        write_i8(a, v - 1);
    }
}

/// LLIST::get_next for a list.
pub(crate) unsafe fn awol_llist_next(list: usize) -> usize {
    exe::llist_get_next(list)
}

/// Resets an LLIST cursor (+0xa) to the first node (+2).
pub(crate) unsafe fn awol_llist_rewind(list: usize) {
    write_u32(list + llist::CURSOR, read_usize(list + 2) as u32);
}

/// True if today is after date d (day signed +0, year unsigned +2).
pub(crate) unsafe fn date_has_passed(d: usize) -> bool {
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let a = read_i16(d + 2) as u16;
    ano > a || (ano == a && dia > read_i16(d))
}

/// True if date d plus `semanas` weeks (Date add_weeks) has passed.
pub(crate) unsafe fn weeks_after_have_passed(d: usize, weeks: u32) -> bool {
    let mut t = [0u8; 8];
    date_has_passed(exe::date_add_weeks(d, &mut t, weeks))
}

/// Ends an AWOL case: flips bit 2 of the person's mark byte and removes the record from the AWOL list (this + 4).
pub(crate) unsafe fn close_awol_case(this: usize, r: usize) {
    let m = read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(read_i32(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m ^= person_flags::MISSED_TRAINING as u8;
    covered!("cpw_7 absence removed from the list");
    exe::llist_remove(read_usize(this + 4), r); // cpw7: removes the case from the list
}

/// Calls AWOL miss_training for a person.
pub(crate) unsafe fn awol_miss_training(this: usize, person: Staff) {
    exe::awol_miss_training(this, person);
}

/// `AWOL::daily_update()`: resolves the day's AWOL cases, missed trainings and late arrivals. Called every day by the
/// game loop (0x5cccd0). Three lists, each with its cursor at `llist::CURSOR`.
///
/// (1) Absences ([this]): the first one whose return date (+4) has passed ends (`exe::awol_end_absence`).
///
/// (2) Missed trainings ([this+4]): the first one a week past its date (+4) is resolved (`resolve_awol_case`). Repeat
/// case (`awol_missed_training::REPEAT_CASE`), with two draws coming out zero, by the person's professionalism
/// (`staff::PROFESSIONALISM`) and temperament (`staff::TEMPERAMENT`): both drop by a draw and the person misses
/// training again (`awol_miss_training`). New case: the teammates at the club (50 slots, `club.squad()`) under the
/// offender's contract may ask to leave (`exe::staff_contract_set_flags` with 2) or lose professionalism, work rate
/// (`.work_rate()` of the player) and temperament; and the offender, when draw(professionalism) is below draw(15) + 1,
/// loses professionalism and work rate (`player_sheet::WORK_RATE`), misses training again and falls out with the club
/// (`exe::media_create_player_media_story` with 8).
///
/// (3) Late arrivals ([this+8]): the first one a week past (+8) that wins the draws (repeat case by
/// `awol_fitness_incident::REPEAT_CASE`) loses professionalism and temperament or work rate
/// (`player_sheet::WORK_RATE`).
///
/// Each resolved case leaves its list (`exe::llist_remove`) and bit 2 of the flag (`person_flags::MISSED_TRAINING`)
/// flips. A list that ends before the count ends the day.
/// Original: `AWOL::daily_update` (0x4190d0, awol.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn awol_daily_update(this: usize) {
    // (1) the absences
    let l = read_usize(this);
    if l != 0 {
        awol_llist_rewind(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(read_usize(this));
                if r == 0 {
                    return;
                }
                if date_has_passed(r + 4) {
                    exe::awol_end_absence(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this)) as i32 {
                    break;
                }
            }
        }
    }
    // (2) missed training sessions
    let l = read_usize(this + 4);
    if l != 0 {
        awol_llist_rewind(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(read_usize(this + 4));
                if r == 0 {
                    return;
                }
                if weeks_after_have_passed(r + 4, 1) { // cpw7: one week later
                    covered!("cpw_7 absence expired");
                    resolve_awol_case(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this + 4)) as i32 {
                    break;
                }
            }
        }
    }
    // (3) the delays
    let l = read_usize(this + 8);
    if l == 0 {
        return;
    }
    awol_llist_rewind(l);
    if read_i16(l) <= 0 {
        return;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(read_usize(this + 8));
        if r == 0 {
            return;
        }
        if weeks_after_have_passed(r + 8, 1) {
            let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE)));
            let resolveu = if read_u8(r + awol_fitness_incident::REPEAT_CASE) != 0 {
                if person.player().is_null() {
                    true
                } else if friendly_random((person.professionalism() as u8) as i8 as i32) == 0 && friendly_random((person.temperament() as u8) as i8 as i32) == 0 {
                    maybe_drop_attribute(person.addr() + staff::PROFESSIONALISM, true);
                    maybe_drop_attribute(person.addr() + staff::TEMPERAMENT, true);
                    true
                } else {
                    false
                }
            } else {
                let a = friendly_random((person.professionalism() as u8) as i8 as i32);
                let b = friendly_random(5);
                if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
                    maybe_drop_attribute(person.addr() + staff::PROFESSIONALISM, true);
                    let j = person.player();
                    if !j.is_null() {
                        maybe_drop_attribute(j.addr() + player_sheet::WORK_RATE, true);
                    }
                    true
                } else {
                    false
                }
            };
            if resolveu {
                exe::llist_remove(read_usize(this + 8), r);
                return;
            }
        }
        i += 1;
        if i >= read_i16(read_usize(this + 8)) as i32 {
            return;
        }
    }
}

/// Resolves an AWOL case one week on: a repeat case may drop professionalism and temperament; a new one involves
/// teammates' reactions; then closes it.
pub(crate) unsafe fn resolve_awol_case(this: usize, r: usize) {
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE)));
    if read_u8(r + awol_missed_training::REPEAT_CASE) != 0 {
        // o caso repetido
        if !person.player().is_null() {
            covered!("cpw_7 repeated case");
            if friendly_random((person.professionalism() as u8) as i8 as i32) == 0 && friendly_random((person.temperament() as u8) as i8 as i32) == 0 { // cpw7: the random draws of the repeated case
                maybe_drop_attribute(person.addr() + staff::PROFESSIONALISM, true);
                maybe_drop_attribute(person.addr() + staff::TEMPERAMENT, true);
                close_awol_case(this, r);
                awol_miss_training(this, person);
                return;
            }
        }
        return close_awol_case(this, r);
    }
    // the new case: the club’s teammates
    covered!("cpw_7 new case");
    let k = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_u32(G_CONTRACT_LOOKUP_RESULT, k.addr() as u32);
    let club = Club(if !k.is_null() { read_usize(G_CLUB_TABLE).wrapping_add(k.club().wrapping_mul(club::SIZE)) } else { 0 });
    if !club.is_null() {
        for v in 0..0x32usize {
            let s = Staff(club.squad(v));
            if s.is_null() || s == person || s.player().is_null() {
                continue;
            }
            let ct = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
            if ct.is_null() {
                continue;
            }
            if exe::staff_contract_get_flags(ct) & 2 == 0 && (s.professionalism() as u8) as i8 > 0xf && (s.ambition() as u8) as i8 > 0xa {
                let a = friendly_random((s.professionalism() as u8) as i8 as i32);
                let b = friendly_random(0x1e);
                let f = ct.squad_status() & 0xf0;
                if a > b && (f == 0x10 || f == 0x20) && (s.age() as u8) as i8 > 0x17 && s.player().current_reputation() > 0x1482 {
                    exe::staff_contract_set_flags(ct, 2);
                }
            }
            if (s.age() as u8) as i8 >= 0x15 || s.player().current_reputation() <= 0x1482 {
                continue;
            }
            maybe_drop_attribute(s.addr() + staff::PROFESSIONALISM, true);
            let j = s.player();
            if (j.work_rate() as u8) as i8 > 1 && friendly_random((s.professionalism() as u8) as i8 as i32) == 0 {
                j.set_work_rate(j.work_rate().wrapping_sub(1));
            }
            maybe_drop_attribute(s.addr() + staff::TEMPERAMENT, true);
        }
    }
    // o faltoso
    let j = person.player();
    if !j.is_null() {
        let a = friendly_random((person.professionalism() as u8) as i8 as i32);
        let b = friendly_random(0xf);
        if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
            maybe_drop_attribute(person.addr() + staff::PROFESSIONALISM, false);
            maybe_drop_attribute(person.player().addr() + player_sheet::WORK_RATE, false);
            close_awol_case(this, r);
            awol_miss_training(this, person);
            if !club.is_null() && !person.player().is_null() {
                exe::media_create_player_media_story(read_usize(G_MEDIA), person, club, 8, 0, 0);
            }
            return;
        }
    }
    close_awol_case(this, r);
}

// simplification: rolls random numbers, removes and frees list nodes (0x68d1f0), gives return news (0x419710) and absence news (0x41a360): only
// the original; the copy takes the branches.

/// `AWOL::has_missed_training(person)`: returns bit 2 of the person's flag (`person_flags::MISSED_TRAINING`); without a
/// contract, clears the miss first. Called by the daily player update (0x5a69b0) and by the screens.
///
/// Null person or beyond the count (`G_STAFF_COUNT`): false. With bit 2 of her flag (`awol_fields::PERSON_FLAGS` + id)
/// set and no contract (`exe::contract_manager_get_contract`, stored in `G_CONTRACT_LOOKUP_RESULT`), clears the miss
/// (`exe::awol_clear_missed_training`) first. Returns bit 2 of the flag.
/// Original: `AWOL::has_missed_training` (0x418ff0, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn has_missed_training(this: usize, person: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return id as u32 & 0xffffff00;
    }
    if read_u8(read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(id as usize)) & (person_flags::MISSED_TRAINING as u8) != 0 {
        let k = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
        write_u32(G_CONTRACT_LOOKUP_RESULT, k.addr() as u32);
        if k.is_null() || read_usize(G_CLUB_TABLE).wrapping_add(k.club().wrapping_mul(club::SIZE)) == 0 {
            exe::awol_clear_missed_training(this, person);
        }
    }
    (read_u8(read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(read_i32(person) as usize)) & (person_flags::MISSED_TRAINING as u8)) as u32
}

/// `AWOL::has_fitness_incident(person)`: true when the person has a record in the late arrival list. Called by the
/// daily player update (0x5a69b0).
///
/// Null person, beyond the count (`G_STAFF_COUNT`) or no late arrival list ([this+8]): false; otherwise walks the list
/// (cursor at `llist::CURSOR`) looking for the person's record (id at +0). True in the low byte; the high bytes as the
/// original leaves them in the register.
/// Original: `AWOL::has_fitness_incident` (0x419070, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn has_fitness_incident(this: usize, person: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return id as u32 & 0xffffff00;
    }
    let l = read_usize(this + 8);
    if l == 0 {
        return 0;
    }
    awol_llist_rewind(l);
    if read_i16(l) <= 0 {
        return l as u32 & 0xffffff00;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(read_usize(this + 8));
        if r == 0 {
            return 0;
        }
        if read_i32(r) == read_i32(person) {
            return (read_i32(person) as u32 & 0xffffff00) | 1;
        }
        i += 1;
        let n = read_i16(read_usize(this + 8)) as i32;
        if i >= n {
            return n as u32 & 0xffffff00;
        }
    }
}

// ---- the originals that awol_daily_update calls and that read leftovers from the private stack (259th batch bench) ----

/// The dry run of the comparers of `miss_training`, `end_absence` and `set_flags`.
///
/// The copy stops before the calls that allocate, send news or cannot run twice, and leaves in `MISS_TRAINING_DECISION`
/// what it would have done.
pub(crate) static MISS_TRAINING_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static MISS_TRAINING_DECISION: AtomicU32 = AtomicU32::new(0);

/// Club record of contract k (id +4, G_CLUB_TABLE), or 0.
pub(crate) unsafe fn awol_club_of_contract(k: Contract) -> usize {
    if k.is_null() {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(k.club().wrapping_mul(club::SIZE))
}

/// Byte +0xc (repeated case) of the absence record.
///
/// `miss_training` does not write it: the 13-byte record is built in a local and +0xc keeps whatever the previous call left in
/// the dword E−0x10 of the stack (E: esp on entry). On the original routine's normal path, through
/// game_check_players_missing_training (`game_check_players_missing_training`), the previous call that reaches that depth is `exe::date_days_since` (days since
/// arrival), which stores its `this` there, the game date 0xae2c90: the low byte is 0x90.
///
/// The copy uses that value (a repeated case); the comparer does not compare it (the original's value is genuine
/// residue).
pub(crate) const AWOL_RESIDUE_BYTE: u8 = 0x90;

/// `AWOL::miss_training(person)`: the player misses training (or, if very unhappy, may go AWOL). Called by the day's
/// missed trainings (`game_check_players_missing_training`) and by awol_daily_update. SEH frame only for the local
/// date, no destructor.
///
/// Player (`.player()`) within the count, with a contract (`exe::contract_manager_get_contract`, in
/// `G_CONTRACT_LOOKUP_RESULT`) at a club and without flag 2 (`person_flags::MISSED_TRAINING`): if the contract has
/// morale (`contract::MORALE`) at or below −100 and bit 0x2000 (then 0x800) in `contract::GRIEVANCES`, and
/// draw(professionalism + 10) and draw(`.loyalty()` + 10) both come out zero, he goes AWOL (`exe::awol_start_absence`
/// with 0, 0).
///
/// Otherwise: sets bit 2, posts the missed training news (`exe::awol_post_missed_training_news`, with the contract's
/// club), byte +0xa of the 79-byte row (`G_STAFF_STATE_TABLE`, `staff_state::SIZE`) adds draw(10), the condition
/// (`exe::injury_get_condition`) drops by 25 × (21 − the player's `.natural_fitness()`) + draw + 100
/// (`exe::injury_set_match_condition`), and the record (person, today, case) joins the missed training list ([this+4],
/// `exe::llist_add_tail`); the case byte, unset in the original, goes 0x90 (`AWOL_RESIDUE_BYTE`). AI club
/// (`exe::human_manager_is_human_club`) with a manager (`club.manager()`, `.non_player()`) and draw(his
/// `.discipline()`) not zero: the fine (`exe::fine_fine_player`, period 1 if draw(manager rating
/// `exe::non_player_attribute_rating` with `.man_handling()`) beats draw(5), else 2; reason 1).
/// Original: `AWOL::miss_training` (0x41a360, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn miss_training(this: usize, person: usize) {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
    let club = Club(awol_club_of_contract(Contract(k)));
    if k == 0 || club.is_null() {
        return;
    }
    let id = read_i32(person.addr());
    let m = read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(id as usize) as *mut u8;
    if *m & (person_flags::MISSED_TRAINING as u8) != 0 {
        return;
    }
    let k2 = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    if !k2.is_null() && (k2.morale() as u8) as i8 <= -100 {
        for bit in [0x2000u32, 0x800] {
            if (k2.grievances() as usize) as u32 & bit != 0
                && friendly_random((person.professionalism() as u8) as i8 as i32 + 10) == 0
                && friendly_random((person.loyalty() as u8) as i8 as i32 + 10) == 0
            {
                covered!("cpw_7 vanished instead of missing");
                if MISS_TRAINING_DRY_RUN.load(Relaxed) {
                    MISS_TRAINING_DECISION.store(1, Relaxed);
                    return;
                }
                exe::awol_start_absence(this, person, (0) as usize, 0);
                return;
            }
        }
    }
    covered!("cpw_7 marked the absence");
    if MISS_TRAINING_DRY_RUN.load(Relaxed) {
        MISS_TRAINING_DECISION.store(2, Relaxed);
        return;
    }
    *m |= person_flags::MISSED_TRAINING as u8;
    let k3 = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k3 as u32);
    exe::awol_post_missed_training_news(person, awol_club_of_contract(Contract(k3)));
    let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(person.addr()) as usize).wrapping_mul(staff_state::SIZE)).wrapping_add(0xa) as *mut u8;
    let r = friendly_random(10);
    *row = (*row).wrapping_add(r as u8);
    let c = exe::injury_get_condition(read_usize(G_INJURY), person, 0);
    // the original’s calculations are 32-bit with high bytes from other values in the register, but only the low word
    // arrives (the condition is short in 0x62bd10)
    let queda = 0x15i16.wrapping_sub((person.player().natural_fitness() as u8) as i8 as i16).wrapping_mul(25);
    let r = friendly_random(queda as i32);
    let new = (c as u16).wrapping_sub((queda as u16).wrapping_add(r as u16).wrapping_add(0x64));
    exe::injury_set_match_condition(read_usize(G_INJURY), person, new as i16, 0);
    let mut reg = [0u8; 13];
    reg[..4].copy_from_slice(&read_i32(person.addr()).to_le_bytes());
    reg[4..12].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    reg[12] = AWOL_RESIDUE_BYTE;
    exe::llist_add_tail(read_usize(this + 4), reg.as_ptr() as usize, 0xd);
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let manager = club.manager();
    if manager.is_null() {
        return;
    }
    let np = manager.non_player();
    if np.is_null() || friendly_random(np.discipline() as i8 as i32) == 0 {
        return;
    }
    let a = friendly_random((exe::non_player_attribute_rating(np, (np.man_handling() as u32) as i8) as i8) as i32);
    let b = friendly_random(5);
    covered!("cpw_7 fine for absence");
    exe::fine_fine_player(read_usize(G_FINE), person, if a > b { 1 } else { 2 }, 1);
}

/// `AWOL::end_absence(record)`: ends an absence. Called by awol_daily_update.
///
/// With the person's flag (+0 id; `awol_fields::PERSON_FLAGS` + id) not zero: if she has a contract
/// (`exe::contract_manager_get_contract`, in `G_CONTRACT_LOOKUP_RESULT`) at a club, the return news
/// (`exe::awol_post_news` with the person's club `club_job()`, 0 and −1); flips bit 1 of the flag
/// (`person_flags::ABSENT_WITHOUT_LEAVE`; the original's `or 1` never gives zero) and removes the record from the
/// absence list ([this], `exe::llist_remove`). Does not read an unset local.
/// Original: `AWOL::end_absence` (0x419710, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn end_absence(this: usize, r: usize) {
    if r == 0 {
        return;
    }
    let m = read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(read_i32(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    covered!("cpw_7 end of absence");
    let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
    if k != 0 && awol_club_of_contract(Contract(k)) != 0 {
        if MISS_TRAINING_DRY_RUN.load(Relaxed) {
            MISS_TRAINING_DECISION.store(1, Relaxed);
        } else {
            let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE)));
            exe::awol_post_news(p, p.club_job(), 0, 0xffffffff);
        }
    }
    let m = read_usize(this + awol_fields::PERSON_FLAGS).wrapping_add(read_i32(r) as usize) as *mut u8;
    *m ^= person_flags::ABSENT_WITHOUT_LEAVE as u8;
    if MISS_TRAINING_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::llist_remove(read_usize(this), r);
}

/// `AWOL::report_poor_fitness(player, club)`: records a fitness incident and handles the reaction. Called by the daily
/// person update (0x5ce2d0: the day's big drop in condition). SEH frame for the date and the local news, no destructor.
///
/// The record (player, club, today and a byte that in the original is stack leftover) joins the incident list
/// ([this+8], `exe::llist_add_tail`, 0x11 bytes).
///
/// Human-run club (`club_has_human_manager`): one coach from the 5 slots (`club::COACHES`, draw(5), until one is found
/// or draw(20) comes out zero) and the news 0x138f to the club (the player, draw(2), his keys and the coach's, or −1).
///
/// AI club (`exe::human_manager_is_human_club`) with a manager (`club::MANAGER`, `.non_player()`): draw(his
/// `.discipline()`) above draw(10) leads to the fine (`exe::fine_fine_player`, period 1 if draw(rating
/// `exe::non_player_attribute_rating` with `non_player::MAN_HANDLING`) beats draw(5), else 2; reason 1).
/// Original: `AWOL::report_poor_fitness` (0x41add0, awol.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn report_poor_fitness(this: usize, player: usize, club: usize) {
    let player = Staff(player);
    let mut reg = [0u8; 0x11];
    exe::date_date_54a110(reg.as_mut_ptr() as usize + 8);
    if player.is_null() || club == 0 {
        return;
    }
    reg[..4].copy_from_slice(&read_u32(player.addr()).to_le_bytes());
    reg[4..8].copy_from_slice(&read_u32(club).to_le_bytes());
    let mut temporary = [0u8; 8];
    exe::date_operator_3d(reg.as_mut_ptr() as usize + 8, temporary.as_mut_ptr() as usize, G_CURRENT_DATE);
    // simplification: byte +0x10 of the record is stack leftover in the original; the copy writes 0 (nobody reads it)
    exe::llist_add_tail(read_usize(this + 8), reg.as_ptr() as usize, 0x11);
    if club_has_human_manager(club) {
        let mut coach;
        loop {
            let r = friendly_random(5);
            coach = read_usize(club.wrapping_add((r as usize).wrapping_mul(4)) + club::COACHES);
            if friendly_random(0x14) == 0 || coach != 0 {
                break;
            }
        }
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        exe::new_newsitem(nt, 0x138f, 0);
        set_news_slot(nt, 0, read_u32(player.addr()));
        set_news_slot(nt, 1, read_u32(club));
        let r = friendly_random(2);
        set_news_slot(nt, 2, r as u32);
        set_news_slot(nt, 3, read_u32(player.first_name() + names::ID));
        set_news_slot(nt, 4, read_u32(player.second_name() + names::ID));
        set_news_slot(nt, 5, read_u32(player.common_name() + names::ID));
        if coach != 0 {
            set_news_person_slots(nt, 6, Staff(coach));
        } else {
            for k in 6..10 {
                set_news_slot(nt, k, u32::MAX);
            }
        }
        exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let manager = Staff(read_usize(club + club::MANAGER));
    if manager.is_null() {
        return;
    }
    let np = manager.non_player();
    if np.is_null() {
        return;
    }
    covered!("cpz_9 fine evaluated");
    let a = friendly_random((np.discipline() as i8) as i32);
    let b = friendly_random(10); // cpz9: the fine random draw
    if a <= b {
        return;
    }
    let np = read_usize(read_usize(club + club::MANAGER) + staff::NON_PLAYER);
    covered!("cpz_9 fine by form");
    // simplification: the original pushes ecx with byte +0x19 only in cl (the high ones are leftover); 0x544410 only reads the byte
    let c =friendly_random((exe::non_player_attribute_rating(np, (read_u8(np + non_player::MAN_HANDLING) as u32) as i8) as i8) as i32);
    let e = friendly_random(5); // cpz9: the deadline random draw
    let deadline = if c > e { 1 } else { 2 }; // cpz9: the fine deadline
    exe::fine_fine_player(read_usize(G_FINE), player, deadline, 1);
}

/// `AWOL::is_player_awol(p, so_read)`: returns bit 0 of the person's flag (`person_flags::ABSENT_WITHOUT_LEAVE`);
/// without `so_read` and without a contract or club, the absence ends first. Called by the squad selection, training,
/// asking price, the staff day and transfers.
///
/// Null person or id beyond `G_STAFF_COUNT`: 0 (only al). With `so_read`, bit 0 of the person's flag
/// (`awol_fields::PERSON_FLAGS` + id). Otherwise, with the bit set and no contract (`injury_main_contract_stored`) or
/// no club on the contract, the absence ends (`exe::awol_clear_awol`); returns the bit read again.
/// Original: `AWOL::is_player_awol` (0x418f60, awol.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_player_awol(this: usize, p: usize, so_read: u32) -> u32 {
    if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let mark = || read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_usize(p)) as u32 & (person_flags::ABSENT_WITHOUT_LEAVE as u32);
    if so_read == 0 && mark() != 0 {
        let c = injury_main_contract_stored(p);
        if c.is_null() || injury_club_of_contract(c.addr()) == 0 {
            covered!("cqb_5 absence ends without club");
            exe::awol_clear_awol(this, p);
        }
    }
    mark()
}

/// Writes the headline or the body of the AWOL news (slots: 0 the player, 1..3 his name ids, 4 another club, 5 his
/// club, 6 the kind, 7 days away, 8 the nation he went to).
///
/// The headline (name in style 5) by kind: 0 "<player> returns to <club>", 1 "misses training" when the player's id is
/// a multiple of 3, else "goes AWOL", 2 by the id modulo 3 "in transfer row", "misses training" or "goes AWOL", 4
/// "fails to return from leave"; the club's link. The body (style 1) differs when the player's club is the screen's
/// human manager's: kind 0 he is back (and the squad is unhappy), 1 he refuses to play and wants a move, 2 he missed
/// training after being denied a move to the other club, 4 he failed to return from leave: to the nation, with
/// "missed his flight" under 7 days away, or the agent's message with a nation, or without one; the other managers
/// read the same with the club's name first. Then the player's link and the two clubs' links (long name).
/// Defect of the original: kind 3 (and above 4) formats nothing, so the text is whatever the shared buffer 0xde1f64
/// last held; the headline reads slot 6 twice and never slot 7.
/// Original: `write_awol_news_text` (0x419c60, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_awol_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = awol_news_slots(news);
        let kind = exe::newsitem_get_slot(news, 6);
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        let by_id = s[0] as i32 % 3;
        match kind {
            0 => {
                let (club, g) = awol_club(s[5]);
                // "<player> returns to <club>"
                fmt(buf, 0x98811c, g, g, np, club)
            }
            1 if by_id == 0 => fmt(buf, 0x9881d4, np), // "<player> misses training"
            2 if by_id == 0 => fmt(buf, 0x9881a0, np), // "<player> in transfer row"
            2 if by_id == 1 => fmt(buf, 0x9881d4, np),
            1 | 2 => fmt(buf, 0x988170, np), // "<player> goes AWOL"
            4 => fmt(buf, 0x988208, np), // "<player> fails to return from leave"
            _ => 0,
        };
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        if s[5] as i32 > -1 {
            exe::text_add_link(out, (club_at(s[5] as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(s[5] as i32 as isize));
        }
        return;
    }
    covered!("sx05_5 awol body");
    let s: [u32; 9] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (other, club, kind, days, nation) = (s[4], s[5], s[6], s[7] as i32, s[8]);
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let (cn, g) = awol_club(club);
    if awol_is_human_club(club) {
        covered!("sx05_5 awol at the human club");
        match kind {
            0 => fmt(buf, 0x988248, np),
            1 => fmt(buf, 0x988310, g, g, np, cn),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x9883e0, og, og, g, g, np, on, cn)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988698, ng, ng, g, g, np, nn, cn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x9885a4, ng, ng, np, nn)
            }
            4 => fmt(buf, 0x9884cc, np),
            _ => 0,
        };
    } else {
        match kind {
            0 => fmt(buf, 0x9887bc, g, g, cn, np),
            1 => fmt(buf, 0x9888b4, g, g, cn, np),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x988990, g, g, og, og, cn, np, on)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988ccc, g, g, ng, ng, cn, np, nn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988b9c, g, g, ng, ng, cn, np, nn)
            }
            4 => fmt(buf, 0x988a88, g, g, cn, np),
            _ => 0,
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    for c in [other, club] {
        if c as i32 > -1 {
            exe::text_add_link(out, (club_at(c as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(c as i32 as isize));
        }
    }
}

/// Writes the headline or the body of the missed training news (slots: 0 the player, 1..3 his name ids, 4 his club,
/// 5 the kind, 6 the absent flag, 7 the coach who reports (-1 none), 8..10 the coach's name ids).
///
/// The headline (slots 0..3, 5 and 6; name in style 5): "<player> absent from training" with the absent flag, else
/// "rumoured to have missed training" for kind 0 or "disciplined by club"; the player's link. The body (style 1): at
/// the screen's human manager's club the chief coach (or the named coach) reports the player did not turn up (kind
/// 0), was absent (1) or failed to report (other); elsewhere the club's board is losing patience (no coach), or the
/// manager is losing patience (kind 0) or fined him two weeks' (1) or one week's wages (other). Then the player's
/// link, the coach's link and the club's link (long name). Slot 6 is read in the body and not used.
/// Original: `write_missed_training_news_text` (0x41a8e0, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_missed_training_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = {
            let mut s = [0u32; 7];
            for k in [0u32, 1, 2, 3, 5, 6] {
                s[k as usize] = exe::newsitem_get_slot(news, k);
            }
            s
        };
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        if s[6] != 0 {
            fmt(buf, 0x988e98, np); // "<player> absent from training"
        } else if s[5] == 0 {
            fmt(buf, 0x988e54, np); // "<player> rumoured to have missed training"
        } else {
            fmt(buf, 0x988e20, np); // "<player> disciplined by club"
        }
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 missed training body");
    let s: [u32; 11] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (club, kind, coach) = (s[4], s[5], s[7] as i32);
    let coach_ids = [s[8], s[9], s[10]];
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let mut coach_name = [0u8; 0x190];
    if coach != -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
    }
    let cp = coach_name.as_ptr() as usize;
    if awol_is_human_club(club) {
        covered!("sx05_5 missed training at the human club");
        let (alone, with_coach) = match kind {
            0 => (0x989250, 0x9891d0),
            1 => (0x98933c, 0x9892c0),
            _ => (0x98942c, 0x9893a8),
        };
        if coach == -1 {
            fmt(buf, alone, np);
        } else {
            fmt(buf, with_coach, cp, np);
        }
    } else {
        let (cn, g) = awol_club(club);
        match (coach, kind) {
            (-1, _) => fmt(buf, 0x988ed0, g, g, 0x64, cn, np),
            (_, 0) => fmt(buf, 0x988f6c, g, g, 0x64, cn, cp, np),
            (_, 1) => fmt(buf, 0x989030, g, g, cn, cp, np),
            _ => fmt(buf, 0x989100, g, g, cn, cp, np),
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    if coach != -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
    let c = club_at(club as i32 as isize);
    exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
}

/// Writes the headline or the body of the news that a coach worries about a player's fitness after he turned up late
/// for training (slots: 0 the player, 1 his club, 2 the worry flag, 3..5 his name ids, 6 the coach (-1 none), 7..9 the
/// coach's name ids). The text goes through a local buffer, not 0xde1f64.
///
/// The headline (style 5): "<player> fitness worry" with the flag, else "<player> lacking fitness"; the player's
/// link. The body (style 1): with a coach, "<club> coach <coach> has voiced concerns over <player>'s general fitness"
/// (flag) or "reported that <player> turned up late"; without one, "various members of the <club> backroom staff..."
/// (flag) or "<player>'s apparent lack of match fitness... has disappointed senior members of <club>'s playing staff".
/// Then the player's link, the club's link (short name) and the coach's link.
/// Original: `write_fitness_worry_news_text` (0x41b030, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_fitness_worry_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let mut text = [0u8; 0x1378];
    let tp = text.as_mut_ptr() as usize;
    let s: [u32; 10] = awol_news_slots(news);
    let (club, worry, coach) = (s[1], s[2], s[6] as i32);
    let ids = [s[3], s[4], s[5]];
    if full == 0 {
        let name = awol_news_name(ids, s[0], 5);
        if worry != 0 {
            fmt(tp, 0x989820, 0x64, name.as_ptr() as usize); // "<player>{} fitness worry"
        } else {
            fmt(tp, 0x9897ec, name.as_ptr() as usize); // "<player> lacking fitness"
        }
        exe::text_set_string(out, tp as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 fitness worry body");
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let coach_ids = [s[7], s[8], s[9]];
    let (cn, g) = awol_club(club);
    let mut coach_name = [0u8; 0x190];
    if coach > -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
        let cp = coach_name.as_ptr() as usize;
        if worry != 0 {
            fmt(tp, 0x989718, g, g, 0x64, cn, cp, np);
        } else {
            fmt(tp, 0x98963c, g, g, cn, cp, np);
        }
    } else if worry != 0 {
        fmt(tp, 0x98956c, g, g, 0x64, cn, np);
    } else {
        covered!("sx05_5 fitness worry without a coach");
        fmt(tp, 0x98949c, 0x64, g, g, np, cn);
    }
    exe::text_set_string(out, tp as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    exe::text_add_link(out, cn as *const u8, 0x7d5, club_at(club as i32 as isize));
    if coach > -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_write_awol_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_AWOL_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_awol_news_text(this, news, full, t),
        || format!("write_awol_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_missed_training_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_MISSED_TRAINING_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_missed_training_news_text(this, news, full, t),
        || format!("write_missed_training_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_fitness_worry_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_FITNESS_WORRY_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_fitness_worry_news_text(this, news, full, t),
        || format!("write_fitness_worry_news_text(news {:#x}, full {})", news, full));
}

/// The slots of an AWOL-family news item, read in order 0..`n` (`exe::newsitem_get_slot`, as the original does).
pub(crate) unsafe fn awol_news_slots<const N: usize>(news: usize) -> [u32; N] {
    let mut s = [0u32; N];
    for (k, v) in s.iter_mut().enumerate() {
        *v = exe::newsitem_get_slot(news, k as u32);
    }
    s
}

/// A person's name from the three name ids of a news item (`exe::database_format_name_from_ids`, 0x190 bytes) in the
/// given style.
pub(crate) unsafe fn awol_news_name(ids: [u32; 3], id: u32, style: u32) -> [u8; 0x190] {
    let mut name = [0u8; 0x190];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    name
}

/// Links a person's name when his names still match the news item's ids.
pub(crate) unsafe fn awol_link_person(out: usize, name: &[u8], ids: [u32; 3], id: u32) {
    let person = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, person);
    }
}

/// The club whose club id is the screen's human manager's club job, as the AWOL texts test it.
pub(crate) unsafe fn awol_is_human_club(club_id: u32) -> bool {
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize));
    !manager.is_null() && !manager.club_job().is_null() && read_u32(manager.club_job().addr()) == club_id
}

/// A club's short name and the gender byte of it (sign-extended, as the texts pass it).
pub(crate) unsafe fn awol_club(id: u32) -> (usize, u32) {
    let club = club_at(id as i32 as isize);
    (club.addr() + club::SHORT_NAME, (club.short_gender_name() as i8) as i32 as u32)
}

/// A nation's short name and the gender byte of it.
pub(crate) unsafe fn awol_nation(id: u32) -> (usize, u32) {
    let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE)));
    (nation.addr() + nation::SHORT_NAME, (nation.short_gender_name() as i8) as i32 as u32)
}

/// Builds the AWOL state: everything 0; with a save, `AWOL::load`, and nothing more when it reads. Without a save, or
/// when the load fails (then its flag table and first list are freed first), a zeroed flag byte per staff member
/// (`exe::crt_calloc`(`G_STAFF_COUNT`, 1)) and three new empty lists. Returns this.
/// Defect of the original: a failed load frees only the first list; the second and third (when it built them) leak.
/// Original: `AWOL::AWOL` (0x418440, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_construct(this: usize, save: usize) -> usize {
    write_usize(this + awol_fields::PERSON_FLAGS, 0);
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, 0);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_AWOL_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx05b_8 awol load failed");
        let flags = read_usize(this + awol_fields::PERSON_FLAGS);
        if flags != 0 {
            exe::crt_free(flags);
            write_usize(this + awol_fields::PERSON_FLAGS, 0);
        }
        let first = read_usize(this);
        if first != 0 {
            exe::drop_llist(first);
            exe::crt_operator_delete(first);
            write_usize(this, 0);
        }
    }
    write_usize(this + awol_fields::PERSON_FLAGS, exe::crt_calloc(read_u32(G_STAFF_COUNT), 1));
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, awol_new_list());
    }
    this
}

/// Loads the AWOL state from a save's awol.dat (`exe::zopen`(save, "awol.dat", 0, 0x16, 4); none, 0): the staff count
/// (zero counts as a failure), the flag table (`exe::crt_malloc` of that many bytes, read in one go), the absences
/// (count, then each 12-byte record appended to a new list), the missed trainings (13 bytes) and, only in saves of a
/// version above 0x3e005 (`G_GAME_DATA_VERSION`, by its absolute value), the fitness incidents (17 bytes; older saves
/// get an empty third list). Any short read closes the file and gives 0; otherwise the file is closed and 1.
/// Original: `AWOL::load` (0x418670, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    let ok = awol_load_lists(this, z);
    exe::zclose(&mut z);
    ok as u8
}

/// Comparer for `awol_construct`: it allocates (and may read the save): only the original runs; then the flag table
/// and the three lists must exist.
pub(crate) unsafe extern "thiscall" fn compare_awol_construct(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_AWOL_CONSTRUCT)(this, save);
    let ok = r == this && read_usize(this + awol_fields::PERSON_FLAGS) != 0 && AWOL_LISTS.iter().all(|&(off, _)| read_usize(this + off) != 0);
    SCOREBOARD.record(ok, || format!("awol_construct(save {:#x}): flags {:#x} lists {:x?}", save, read_usize(this + awol_fields::PERSON_FLAGS),
        AWOL_LISTS.iter().map(|&(off, _)| read_usize(this + off)).collect::<Vec<_>>()));
    r
}

/// Comparer for `awol_load`: the save is read once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_AWOL_LOAD)(this, save)
}

/// Saves the AWOL state to the save's awol.dat as `awol_load` reads it: without a save, the "Error" box (line 0x132)
/// and 0; the file (`exe::zopen`(save, "awol.dat", 1, 0x16, 4); none, the box of line 0x13a and 0, nothing to close);
/// the staff count (`G_STAFF_COUNT`, line 0x141), the flag table (that many bytes, line 0x148) and each of the three
/// lists: its count (the list's signed word, as a dword; lines 0x151, 0x16a, 0x183), then its records walked from the
/// head (`llist::CURSOR` = `llist::HEAD`, `exe::llist_get_next`: a missing record, lines 0x15a, 0x173, 0x18c; a short
/// write, 0x160, 0x179, 0x192) at 12, 13 and 17 bytes. Each failure after the file opens shows the box, closes it and
/// gives 0; otherwise it closes and gives 1.
/// Original: `AWOL::save` (0x418980, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_at(AWOL_CPP, 0x132);
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(AWOL_CPP, 0x13a);
        return 0;
    }
    let failed = awol_save_lists(this, z);
    if let Some(line) = failed {
        show_error_at(AWOL_CPP, line);
    }
    exe::zclose(&mut z);
    failed.is_none() as u8
}

/// Comparer for `awol_save`: the save file is written once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_AWOL_SAVE)(this, save)
}

/// A new empty record list (`exe::new_llist` in 0xe bytes of `exe::crt_operator_new`), 0 when out of memory.
pub(crate) unsafe fn awol_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// The body of `awol_load` once the file is open.
pub(crate) unsafe fn awol_load_lists(this: usize, z: usize) -> bool {
    let mut n = 0u32;
    let np = &mut n as *mut u32 as usize;
    if !awol_read(np, 4, z) || n == 0 {
        return false;
    }
    let flags = exe::crt_malloc(n);
    write_usize(this + awol_fields::PERSON_FLAGS, flags);
    if flags == 0 || exe::zread(flags, 1, n, z) != n || !awol_read(np, 4, z) {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).unsigned_abs();
    for (k, &(off, size)) in AWOL_LISTS.iter().enumerate() {
        if k == 2 {
            write_usize(this + off, awol_new_list());
            if version <= 0x3e005 {
                return true;
            }
            covered!("sx05b_8 fitness incidents read");
            if !awol_read(np, 4, z) {
                return false;
            }
        } else {
            write_usize(this + off, awol_new_list());
        }
        let mut record = [0u8; 0x11];
        for _ in 0..(n as i32).max(0) {
            if !awol_read(record.as_mut_ptr() as usize, size, z) {
                return false;
            }
            exe::llist_add_tail(read_usize(this + off), record.as_ptr() as usize, size);
        }
        if k == 0 && !awol_read(np, 4, z) {
            return false;
        }
    }
    true
}

/// The body of `awol_save` once the file is open: the line of the first failure, if any.
pub(crate) unsafe fn awol_save_lists(this: usize, z: usize) -> Option<u32> {
    if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 {
        return Some(0x141);
    }
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this + awol_fields::PERSON_FLAGS), 1, staff, z) != staff {
        return Some(0x148);
    }
    const LINES: [(u32, u32, u32); 3] = [(0x151, 0x15a, 0x160), (0x16a, 0x173, 0x179), (0x183, 0x18c, 0x192)];
    for (&(off, size), &(count_line, missing_line, write_line)) in AWOL_LISTS.iter().zip(LINES.iter()) {
        let list = read_usize(this + off);
        let count = read_i16(list + llist::COUNT) as i32;
        if exe::zwrite(&count as *const i32 as usize, 4, 1, z) != 1 {
            return Some(count_line);
        }
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut k = 0;
        while k < read_i16(read_usize(this + off) + llist::COUNT) as i32 {
            let record = exe::llist_get_next(read_usize(this + off));
            if record == 0 {
                return Some(missing_line);
            }
            if exe::zwrite(record, size, 1, z) != 1 {
                return Some(write_line);
            }
            k += 1;
        }
    }
    covered!("sx05b_9 awol saved");
    None
}

/// Reads one count or record from awol.dat; false on a short read.
pub(crate) unsafe fn awol_read(dest: usize, size: u32, z: usize) -> bool {
    exe::zread(dest, size, 1, z) == 1
}

/// The AWOL object's three record lists (`LLIST`, 0xe bytes each) and the size of their records: the absences (+0,
/// 12 bytes: staff id and the return date), the missed trainings (+4, 13 bytes) and the fitness incidents (+8, 17
/// bytes); the per-staff flag table is `awol_fields::PERSON_FLAGS`.
pub(crate) const AWOL_LISTS: [(usize, u32); 3] = [(0x0, 0xc), (0x4, 0xd), (0x8, 0x11)];

/// "E:\dev\CM3\cm3 00-01\cm3\code\awol.cpp" (the error boxes) and "awol.dat" (its file in the save).
pub(crate) const AWOL_CPP: usize = 0x9880f4;

pub(crate) const AWOL_DAT: usize = 0x9880e8;

/// `AWOL::load(save)` (0x418670, thiscall, ret 4: al nonzero when read), through its hook.
pub(crate) const FN_AWOL_LOAD: usize = 0x418670;

/// Sends a player AWOL. Nothing without a person, a player sheet, a staff id below `G_STAFF_COUNT`, or when his flag
/// already has bit 1 (`person_flags::ABSENT_WITHOUT_LEAVE`). Otherwise the bit is set and his current, home and world
/// reputations are each cut by a random 0 to 40 percent (`awol_cut_reputation`, one draw each, in that order). At his
/// club: he dislikes the manager; unless a human runs the club (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0)),
/// the manager dislikes him; each squad member but him dislikes him when a draw of his loyalty is not 0 and a draw of
/// his professionalism is 0; the chairman dislikes him (`exe::staff_add_disliked_staff`(who, whom, 1)). The absence
/// (12 bytes: his staff id and the return date, today plus 2 to 32 days for kind 4, else 21 to 170) goes to the
/// absences list. Then the news (`awol_post_news`) with `club` (or his own): kind 2 for kind 1, kind 4 with the days
/// for kind 4, kind 1 otherwise (days −1).
/// Original: `AWOL::start_absence` (0x419840, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let flag = read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(person.addr()) as usize;
    if read_u8(flag) & person_flags::ABSENT_WITHOUT_LEAVE as u8 != 0 {
        return;
    }
    covered!("sx05b_11 player goes awol");
    write_u8(flag, read_u8(flag) | person_flags::ABSENT_WITHOUT_LEAVE as u8);
    let sheet = person.player().addr();
    for word in [player_sheet::CURRENT_REPUTATION, player_sheet::HOME_REPUTATION, player_sheet::WORLD_REPUTATION] {
        awol_cut_reputation(sheet + word);
    }
    let own = person.club_job();
    if !own.is_null() {
        exe::staff_add_disliked_staff(person, own.manager(), 1);
        if exe::human_manager_is_human_club(own, 1, 1, 0, 0) == 0 {
            exe::staff_add_disliked_staff(own.manager(), person, 1);
        }
        for k in 0..50 {
            let mate = Staff(own.squad(k));
            if mate.is_null() || mate == person {
                continue;
            }
            if exe::utils_random(mate.loyalty() as i32) != 0 && exe::utils_random(mate.professionalism() as i32) == 0 {
                covered!("sx05b_11 squad mate dislikes him");
                exe::staff_add_disliked_staff(mate, person, 1);
            }
        }
        exe::staff_add_disliked_staff(own.chairman(), person, 1);
    }
    let mut record = [0u8; 12];
    let r = record.as_mut_ptr() as usize;
    write_i32(r, read_i32(person.addr()));
    let mut out = [0u8; 8];
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, G_CURRENT_DATE);
    let days = if kind as u8 == 4 { exe::utils_random(0x1f) + 2 } else { exe::utils_random(0x96) + 0x15 };
    let mut later = [0u8; 8];
    let back = exe::date_add_days(r + 4, later.as_mut_ptr() as usize, days as u32);
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, back);
    exe::llist_add_tail(read_usize(this), r, 0xc);
    let to = if club != 0 { club } else { own.addr() };
    let (news_kind, news_days) = match kind as u8 as i8 {
        1 => (2, u32::MAX),
        4 => (4, days as u32),
        _ => (1, u32::MAX),
    };
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_POST_NEWS);
    post(this, person.addr(), to, news_kind, news_days);
}

/// Posts the AWOL news (type 0xbc5) of a player: nothing without a person, his club, his player sheet or `club`.
/// Slots: 0 his staff id, 1..3 his three name ids, 4 `club`'s id, 5 his club's id, 6 the kind, 7 the days (−1 none),
/// 8 his club's nation id (−1 none). A player with a world reputation above 7500 is news for every manager;
/// otherwise his club's nation (when it has one) and, when his nationality is another nation, that nation.
/// Original: `awol_post_news` (0x419ad0, awol.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    awol_post_news_with(this, person, club, kind, days, &mut |item, to| news_post(item, to));
}

/// Comparer for `awol_start_absence`: the draws, the reputations, the dislikes, the list and the news cannot be
/// repeated; only the original runs, and when it could act (a player, his flag free), the flag must have bit 1 and
/// the absences one more record.
pub(crate) unsafe extern "thiscall" fn compare_awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let p = Staff(person);
    let acts = !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
        && read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize) & 1 == 0;
    let before = read_i16(read_usize(this) + llist::COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_AWOL_START_ABSENCE)(this, person, club, kind);
    if acts {
        let after = read_i16(read_usize(this) + llist::COUNT);
        let flag = read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize);
        SCOREBOARD.record(flag & 1 != 0 && after == before + 1, || format!("awol_start_absence(person {:#x}, kind {}): flag {:#x}, absences {} -> {}", person, kind as u8, flag, before, after));
    }
}

/// Comparer for `awol_post_news`: the original posts; the copy rehearses (`rehearse_news`, no draws) and must post
/// when the original could.
pub(crate) unsafe extern "thiscall" fn compare_awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_AWOL_POST_NEWS);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, person, club, kind, days), || awol_post_news_with(this, person, club, kind, days, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok, || format!("awol_post_news(person {:#x}, kind {}) draws original {:?} rust {:?}, rust posts {:x?}", person, kind as u8, a, b, posts));
}

/// The game's sink: posts `item` (`exe::news_send_to_*`, thiscall on `G_NEWS`).
pub(crate) unsafe fn news_post(item: usize, to: NewsPost) {
    let news = read_usize(G_NEWS);
    match to {
        NewsPost::Club(c) => exe::news_send_to_club_manager(news, item, c),
        NewsPost::Division(d) => exe::news_send_to_division_managers(news, item, d),
        NewsPost::Nation(n) => exe::news_send_to_nation_managers(news, item, n),
        NewsPost::Continent(c) => exe::news_send_to_continent_managers(news, item, c),
        NewsPost::All => exe::news_send_to_all_managers(news, item),
    };
}

/// The comparer of a routine that posts news (it cannot run twice: the inbox allocates): the original runs with its
/// random draws traced, then `rehearsal` (the copy, building the same items without posting them, see `NewsSink`)
/// from the same generator state, without counting; the rehearsal's draws must be the start of the original's (the
/// posting may draw more). Returns whether they are and the two draw lists, for the message.
pub(crate) unsafe fn rehearse_news(original: impl FnOnce(), rehearsal: impl FnOnce()) -> (bool, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let (mut from_original, mut from_copy) = (Vec::new(), Vec::new());
    crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || from_original = crate::rng::trace(original).1,
        || from_copy = crate::rng::trace(|| crate::rng::without_counting(rehearsal)).1);
    (from_original.starts_with(&from_copy), from_original, from_copy)
}

/// One of a player's reputation words cut by a random 0 to 40 percent: × (1 − draw(41) × 0.01), truncated.
pub(crate) unsafe fn awol_cut_reputation(word: usize) {
    let draw = exe::utils_random(0x29);
    let v = ftol((1.0 - draw as f64 * read_f64(rdata::F64_0_01)) * read_i16(word) as f64);
    write_i16(word, v as i16);
}

/// `awol_post_news` with the posts going to `sink`.
pub(crate) unsafe fn awol_post_news_with(_this: usize, person: usize, club: usize, kind: u32, days: u32, sink: NewsSink) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    covered!("sx05b_11 awol news");
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0xbc5, 0);
    let own = person.club_job();
    let nation = own.nation().addr();
    let slots = [
        read_u32(person.addr()),
        read_u32(person.first_name() + names::ID),
        read_u32(person.second_name() + names::ID),
        read_u32(person.common_name() + names::ID),
        read_u32(club),
        read_u32(own.addr()),
        kind as u8 as i8 as i32 as u32,
        days as u16 as i16 as i32 as u32,
        if nation != 0 { read_u32(nation) } else { u32::MAX },
    ];
    for (k, &v) in slots.iter().enumerate() {
        exe::newsitem_set_slot(p, k as i8, v);
    }
    if person.player().world_reputation() > 7500 {
        return sink(p, NewsPost::All);
    }
    if nation != 0 {
        sink(p, NewsPost::Nation(nation));
    }
    let nationality = person.nation().addr();
    if own.nation().addr() != nationality {
        sink(p, NewsPost::Nation(nationality));
    }
}

/// What a news routine does with its items: post them in the game, or (a comparer's rehearsal) only note where.
pub(crate) type NewsSink<'a> = &'a mut dyn FnMut(usize, NewsPost);

/// Where an award or AWOL news item goes: one club's managers, a division's, a nation's, a continent's, or every
/// manager (`News::send_to_*` on `G_NEWS`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NewsPost {
    Club(usize),
    Division(usize),
    Nation(usize),
    Continent(usize),
    All,
}
