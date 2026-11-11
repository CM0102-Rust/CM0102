// ---------- batch %%LEVA%%: the AI's squad assessment (shortlist_manager.cpp) ----------

/// The locals of `assess_club_squad` that live across its stages (the original keeps them in a 0x8a4-byte frame;
/// the stack offsets are in the comments).
struct SquadAssessment {
    this: usize,
    entry: usize,
    club: Club,
    reset: bool,
    flags: u32,            // +0x44: 0x800 at a human club, 0x1802 at an AI club (the selection score's flags)
    partner: u32,          // +0x3c: the linked club (B team) or 0
    comp: usize,           // +0x18: the club's continental competition entry (only tested for null)
    target: i32,           // +0x64: the club's target rating (player_search_get_club_target_rating)
    target_base: i32,      // +0x74: its base, written by the same call
    days_since_start: i32, // +0x68: days since the game started
    stand_in: usize,       // +0x2c: the stand-in manager of a human club
    on_holiday: u32,       // +0x30: the human manager is on holiday
    may_sack: bool,        // +0x60: the shortlist was waiting (ACTIVE 2) at an AI club: idle staff may be sacked
    big_club: bool,        // +0x78: the slots were set up for a big club (status 3 for the first 19)
    rating: i32,           // +0x38: the last selection score (read again, stale, by the pricing of a slot of another status)
    pos_a: [u16; 50],      // +0x80: positions of the free slots of status 2 and 3
    n_a: i8,               // +0x40
    pos_b: [u16; 50],      // +0x468: status 4
    n_b: i8,               // +0x6c
    pos_c: [u16; 50],      // +0x274: status 5
    n_c: i8,               // +0x58
    chosen_a: [usize; 50], // +0x4cc
    chosen_b: [usize; 50], // +0x65c
    chosen_c: [usize; 50], // +0x594
    rest: [usize; 100],    // +0x2d8: the players not chosen yet
    n_rest: i8,            // bl, copied to +0x14
    elsewhere: [usize; 100], // +0xe4: players with a loan contract at another club
    n_elsewhere: i8,       // +0x13
    old: [usize; 50],      // +0x724: players too old for the slots of status 4
    n_old: i8,             // +0x13 again
}

/// The slot `k` (0..40) of the club's shortlist entry (`0x25` bytes each).
unsafe fn assess_slot(entry: usize, k: usize) -> ShortlistSlot {
    ShortlistSlot(read_usize(entry).wrapping_add(k.wrapping_mul(0x25)))
}

/// The slot's status (bits 1-4 of `shortlist_slot::FLAGS`): the original's four masks (`& 0xe5 | 4`, `& 0xe7 | 6`,
/// `& 0xe9 | 8`, `& 0xeb | 0xa`) all equal `& 0xe1 | status << 1`.
unsafe fn assess_set_slot_status(slot: ShortlistSlot, status: u8) {
    slot.set_flags((slot.flags() & 0xe1) | (status << 1));
}

/// Happiness plus `delta`, kept within −100..100 (the original adds in 32 bits and writes the byte).
fn assess_happiness_plus(h: i8, delta: i8) -> i8 {
    let e = h as i32 + delta as i32;
    if e > 100 {
        100
    } else if e < -100 {
        -100
    } else {
        h.wrapping_add(delta)
    }
}

/// `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, thiscall, ret 8) through the exe, so its hook applies.
unsafe fn assess_move_to_first_team(person: Staff, club: Club) {
    const MOVE_TO_FIRST_TEAM: usize = 0x4cc0b0;
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(MOVE_TO_FIRST_TEAM);
    f(read_usize(G_CONTRACT_MANAGER), person.addr(), club.addr());
}

/// `CONTRACT_MANAGER::move_to_reserves` for a person of the club.
unsafe fn assess_move_to_reserves(person: Staff, club: Club) {
    exe::contract_manager_move_to_reserves(read_usize(G_CONTRACT_MANAGER), person, club);
}

/// To the reserves, except a star (`player_sheet::WORLD_REPUTATION` ≥ 5000) at a club with a linked B team
/// (0x83dc47, 0x83dca1, 0x83e82f).
unsafe fn assess_to_reserves_unless_star(s: &SquadAssessment, p: Staff) {
    if s.partner == 0 || p.player().world_reputation() < 5000 {
        assess_move_to_reserves(p, s.club);
    }
}

/// The selection score of `p` for the position mask `pos` (`match_man_score_player_for_selection`, cdecl; the
/// original pushes the mask's word with whatever is in the upper half of the register, and the callee only tests the
/// low bits).
unsafe fn assess_score(s: &SquadAssessment, p: usize, pos: u32, flags: u32) -> i32 {
    exe::match_man_score_player_for_selection(p, pos, flags, s.club, -1, s.stand_in)
}

/// `match_man_assign_players_to_positions` (0x6e2000, cdecl, 15 arguments): the people of `rest` (count `n_rest`, a
/// byte) to the positions `pos` (count `n`, a byte), into `out`, with the manager `manager` (the stand-in at a human
/// club).
unsafe fn assess_pick_for(s: &mut SquadAssessment, which: u8, flags: u32) {
    let human = exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) != 0;
    let manager = if human { s.stand_in } else { s.club.manager().addr() };
    let flags = match which {
        0 if human => flags | 0xc100c0,
        0 => flags | 0xc0,
        1 => flags | 0x1c0,
        _ => flags | 0x100,
    };
    let (pos, n, out): (*const u16, i8, *mut usize) = match which {
        0 => (s.pos_a.as_ptr(), s.n_a, s.chosen_a.as_mut_ptr()),
        1 => (s.pos_b.as_ptr(), s.n_b, s.chosen_b.as_mut_ptr()),
        _ => (s.pos_c.as_ptr(), s.n_c, s.chosen_c.as_mut_ptr()),
    };
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    pick(s.rest.as_mut_ptr() as usize, s.n_rest as u8 as u32, pos as usize, n as u8 as u32, 0, out as usize, flags, s.club.addr(), 0, manager as u32, 1, 0, 0, 0, 0);
    let chosen = match which {
        0 => s.chosen_a,
        1 => s.chosen_b,
        _ => s.chosen_c,
    };
    // whoever was chosen leaves the rest: the last one takes his place (the one moved in is not compared again)
    for &p in chosen.iter().take(n.max(0) as usize) {
        if p == 0 {
            continue;
        }
        let mut i: i8 = 0;
        while i < s.n_rest {
            if s.rest[i as usize] == p {
                let last = (s.n_rest - 1) as usize;
                s.rest[i as usize] = s.rest[last];
                s.n_rest -= 1;
                s.rest[last] = 0;
            }
            i += 1;
        }
    }
}

/// The early checks of `assess_club_squad` (0x83cb40..0x83ccd0). `None`: the routine leaves (ACTIVE 0, or untouched
/// when it was 2 with bids out).
unsafe fn assess_start(this: usize, entry: usize, club: Club, daily: u32, reset: &mut bool) -> Option<(u32, bool)> {
    write_u8(entry + club_shortlist::DAYS, 0);
    let human = |a: u32| exe::human_manager_is_human_club(club, a, 1, 0, 0) != 0;
    if exe::database_is_national_team(club) != 0
        || club.nation().is_null()
        || (human(0) && daily as u8 != 0)
        || club.professional_status() == 3
    {
        write_u8(entry + club_shortlist::ACTIVE, 0);
        return None;
    }
    let mut partner: u32 = 0;
    if exe::club_get_linked_b_team(club, &mut partner, 1) == 0 {
        partner = 0;
    } else if partner == 0 {
        write_u8(entry + club_shortlist::ACTIVE, 0);
        return None;
    }
    let mut may_sack = false;
    if exe::shortlist_manager_count_slots_with_bids(this, club, 0xffff_ffff) != 0 {
        if read_u8(entry + club_shortlist::ACTIVE) == 2 {
            return None;
        }
    } else {
        if read_u8(entry + club_shortlist::ACTIVE) == 2 {
            let h = human(1);
            write_u8(entry + club_shortlist::ACTIVE, 1);
            *reset = true;
            may_sack = !h;
        }
        let division = club.division();
        let not_loaded = club.nation().league_selected() == 0
            || division.is_null()
            || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(division.addr()).wrapping_mul(4))) == 0;
        if not_loaded && club.reputation() < 4000 {
            let a = exe::utils_random(club.reputation() as i32 + 1000);
            let b = exe::utils_random(5000);
            if a < b {
                covered!("mkb_2 small club switches off");
                write_u8(entry + club_shortlist::ACTIVE, 0);
                return None;
            }
        }
    }
    write_u8(entry + club_shortlist::ACTIVE, 1);
    Some((partner, may_sack))
}

/// With `reset` the 40 slots are rebuilt (0x83cdf1..0x83cfe3): the formation (+7 of the entry) from the AI manager's
/// non-player record (3 without one, 0 at a human club or with the holiday option 2), the roles and positions
/// (`shortlist_set_up_formation_slots`), and for each slot its index, the date 1/1/1997 (day 1, year 0x7cd) and its
/// status: slot 0 status 2; a goalkeeper slot (position 1) among 1..23 status 4; otherwise, at an AI club without a B
/// team, a big club (a continental competition and base rating ≥ 6000, or base ≥ 8500) has status 3 up to slot 19 and 5
/// from 20, the others status 2 up to slot 10, 4 up to 19 and 5 from 20; with a B team or at a human club, base < 6000
/// without a B team is like the small ones and the rest status 3 up to slot 18, then 4 with a B team or 5 without.
unsafe fn assess_set_up_slots(s: &mut SquadAssessment) {
    let club = s.club;
    let manager = club.manager();
    let formation: u8 = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        && !manager.is_null()
        && (manager.non_player().is_null() || manager.non_player().formation() != 0)
        && !(s.on_holiday != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 2, exe::database_get_club_manager(club)) != 0)
    {
        let np = club.manager().non_player();
        if np.is_null() {
            3
        } else {
            np.formation() as u8
        }
    } else {
        0
    };
    write_u8(s.entry + 7, formation);
    const SET_UP_FORMATION_SLOTS: usize = 0x83c090; // stdcall, ret 4 (the original passes this in ecx, unused)
    let set_up: unsafe extern "stdcall" fn(usize) = core::mem::transmute(SET_UP_FORMATION_SLOTS);
    set_up(s.entry);
    for i in 0..40i8 {
        let slot = assess_slot(s.entry, i as usize);
        slot.set_index(i);
        let mut date = [0u32; 2];
        let mut scratch = [0u32; 2];
        let d = exe::new_date(date.as_mut_ptr() as usize, 1, 0x7cd);
        exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, d);
        let small = |i: i8| if i >= 11 { if i >= 20 { 5 } else { 4 } } else { 2 };
        let status = if i == 0 {
            2
        } else if i > 0 && i < 24 && slot.position() == 1 {
            4
        } else if s.partner == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
            if (s.comp != 0 && s.target_base >= 6000) || s.target_base >= 8500 {
                s.big_club = true;
                if i >= 20 { 5 } else { 3 }
            } else {
                small(i)
            }
        } else if s.target_base < 6000 && s.partner == 0 {
            small(i)
        } else {
            s.big_club = true;
            if i < 19 {
                3
            } else if s.partner != 0 {
                4
            } else {
                5
            }
        };
        assess_set_slot_status(slot, status);
    }
}

/// The slots are released (0x83cff9..0x83d1c7): a slot without a target, or whose target has no bid
/// (`shortlist_slot::OFFER_REF` −1) and whose date has passed or who retired (job 0x10), loses target, replacement and
/// ceiling (the target, if he is in the manager's person pairs, gets the event 3 in `update_staff_status`); its
/// position goes to the list of its status (2 and 3, 4, 5).
unsafe fn assess_release_slots(s: &mut SquadAssessment) {
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        let t = slot.target_staff_id() as i32;
        if t > -1 {
            if slot.offer_ref() != -1 {
                continue;
            }
            let year = read_u16(slot.date_addr() + 2);
            let today = read_u16(G_CURRENT_DATE + 2);
            let expired = year < today
                || (year == today && read_i16(slot.date_addr()) < read_i16(G_CURRENT_DATE))
                || staff_at(t as isize).job_for_club() == 0x10;
            if !expired {
                continue;
            }
            let pairs = read_usize(s.this);
            if pairs != 0 {
                let key = [read_u32(staff_at(t as isize).addr()), if s.club.is_null() { 0xffff_ffff } else { read_u32(s.club.addr()) }];
                if exe::crt_bsearch(key.as_ptr() as usize, pairs, read_u32(s.this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0usize) != 0 {
                    exe::shortlist_manager_update_staff_status(s.this, staff_at(slot.target_staff_id() as i32 as isize), s.club, 3, 0);
                }
            }
        }
        slot.set_target_staff_id(usize::MAX);
        slot.set_replacement_staff_id(0xffff_ffff);
        slot.set_max_price(0);
        if read_u8(slot.addr()) == 0xb {
            let pos = slot.position() as u16;
            match (slot.flags() >> 1) & 0xf {
                2 | 3 => {
                    s.pos_a[s.n_a as usize] = pos;
                    s.n_a = s.n_a.wrapping_add(1);
                }
                4 => {
                    s.pos_b[s.n_b as usize] = pos;
                    s.n_b = s.n_b.wrapping_add(1);
                }
                5 => {
                    s.pos_c[s.n_c as usize] = pos;
                    s.n_c = s.n_c.wrapping_add(1);
                }
                _ => {}
            }
        }
    }
}

/// The squad is sorted out (0x83d1c7..0x83d474): each player of the club (`database_collect_club_squad`) with a
/// sheet, not retired, not on the club's own shortlist, with a contract at the club, no future transfer and not leaving
/// on a Bosman. With no manager on holiday: the transfer and loan marks are cleared, the squad status becomes 6 under
/// 20, else 7 at an AI club (cleared at a human one), and he goes to the first team; on holiday, with `reset` at an AI
/// club, only the status (6 under 20, cleared otherwise). An injury longer than 150 days (30 when random(recurring
/// level + 1) ≠ 0; 400 when his score reaches the target rating, 300 when it reaches 3/4 of it), or a retirement entry,
/// sends him to the reserves (with a B team only past 300 days out) and clears the status at a human club; the others
/// go to the rest, or to the list of those on loan elsewhere, and with `reset` their happiness becomes 0.
unsafe fn assess_sort_squad(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(s.club, squad.as_mut_ptr(), 0);
    for &p in squad.iter() {
        let p = Staff(p);
        if p.is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p.player().is_null() || p.job_for_club() == 0x10 {
            continue;
        }
        if exe::shortlist_manager_find_club_shortlist_entry(s.this, s.club, p, 0, 0) != 0 {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, s.club));
        if c.is_null() || exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 || c.leaving_on_bosman() != 0 {
            continue;
        }
        if s.on_holiday == 0 {
            c.set_transfer_status(c.transfer_status() & 0xfc);
            if (p.age() as i8) < 20 {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            } else if exe::human_manager_is_human_club(s.club, 0, 1, 0, 0) != 0 {
                c.set_squad_status(c.squad_status() & 0xf);
            } else {
                c.set_squad_status((c.squad_status() & 0xf) | 0x70);
            }
            assess_move_to_first_team(p, s.club);
        } else if s.reset && exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) == 0 {
            if (p.age() as i8) < 20 {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            } else {
                c.set_squad_status(c.squad_status() & 0xf);
            }
        }
        let injury = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), p));
        let level = exe::injury_get_recurring_injury_level(read_usize(G_INJURY), p) as i8;
        let long_out = if exe::utils_random(level as i32 + 1) != 0 {
            !injury.is_null() && injury.days_left() as i32 >= 30
        } else if injury.is_null() {
            false
        } else {
            s.rating = assess_score(s, p.addr(), 0xffff_ffff, s.flags | 0x6c0);
            let days = if s.rating >= s.target {
                400
            } else if s.target as f64 * read_f64(rdata::F64_0_75) <= s.rating as f64 {
                300
            } else {
                150
            };
            injury.days_left() as i32 >= days
        };
        if long_out || exe::player_regen_find_retirement_entry(read_usize(G_PLAYER_REGEN), p, 1) != 0 {
            if s.partner == 0 || (!injury.is_null() && injury.days_left() > 300) {
                assess_move_to_reserves(p, s.club);
            }
            if exe::human_manager_is_human_club(s.club, 0, 1, 0, 0) != 0 {
                c.set_squad_status(c.squad_status() & 0xf);
            }
            continue;
        }
        if exe::contract_manager_get_loan_contract(cm, p) != 0 && p.club_job() != s.club {
            s.elsewhere[s.n_elsewhere as usize] = p.addr();
            s.n_elsewhere = s.n_elsewhere.wrapping_add(1);
        } else {
            s.rest[s.n_rest as usize] = p.addr();
            s.n_rest = s.n_rest.wrapping_add(1);
        }
        if s.reset {
            c.set_happiness(0);
        }
    }
}

/// The squad is picked for the free slots (0x83d474..0x83d8ac): the rest goes to the positions of status 2 and 3,
/// then 4; those on loan elsewhere join the rest; those older than `player_sheet::GOALKEEPER` / 10 + 22 leave it
/// (the swapped-in one is looked at again); the rest goes to the positions of status 5; the old ones come back.
unsafe fn assess_pick_squad(s: &mut SquadAssessment) {
    if s.n_a != 0 {
        assess_pick_for(s, 0, s.flags);
    }
    if s.n_b != 0 {
        assess_pick_for(s, 1, s.flags);
    }
    for k in 0..s.n_elsewhere.max(0) as usize {
        s.rest[s.n_rest as usize] = s.elsewhere[k];
        s.n_rest = s.n_rest.wrapping_add(1);
    }
    s.n_old = 0;
    let mut i: i8 = 0;
    while i < s.n_rest {
        let p = Staff(s.rest[i as usize]);
        if !p.is_null() && (p.age() as i8 as i32) > p.player().goalkeeper() as i32 / 10 + 22 {
            s.old[s.n_old as usize] = p.addr();
            s.n_old = s.n_old.wrapping_add(1);
            let last = (s.n_rest - 1) as usize;
            s.rest[i as usize] = s.rest[last];
            s.n_rest -= 1;
            s.rest[last] = 0;
            i -= 1;
        }
        i += 1;
    }
    if s.n_c != 0 {
        assess_pick_for(s, 2, s.flags);
    }
    for k in 0..s.n_old.max(0) as usize {
        if s.old[k] != 0 {
            s.rest[s.n_rest as usize] = s.old[k];
            s.n_rest = s.n_rest.wrapping_add(1);
        }
    }
}

/// One player left over (0x83d8bf..0x83de6b). See `assess_club_squad` for the rule.
unsafe fn assess_rest_player(s: &mut SquadAssessment, p: Staff) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let club = s.club;
    if exe::contract_manager_get_loan_contract(cm, p) != 0 {
        if p.club_job() != club {
            assess_to_reserves_unless_star(s, p);
        }
        return;
    }
    if p.club_job() != club {
        return;
    }
    let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, club));
    if c.is_null() {
        return;
    }
    let age = p.age() as i8;
    if exe::transfer_manager_is_queued_for_free_transfer(tm, p) != 0 {
        if s.reset {
            c.set_happiness(-100);
        }
        return;
    }
    let age_i = age as i32;
    let threshold = ftol((age_i as f64 * read_f64(rdata::F64_0_13) - read_f64(0x96f890)) * s.target_base as f64 * read_f64(rdata::F64_0_02)) as i16; // f64 1.98
    let cut = s.target_base / 68 - 35;
    let sheet = p.player();
    let level = ftol(
        sheet.current_reputation() as f64 * read_f64(rdata::F64_0_25)
            + sheet.potential_ability() as f64 * read_f64(rdata::F64_12_5)
            + sheet.current_ability() as f64 * read_f64(rdata::F64_12_5)
            + club.reputation() as f64 * read_f64(rdata::F64_0_25),
    );
    let manager = club.manager();
    let rep = club.reputation() as i32;
    if p.job_for_club() == 0xb && exe::match_man_judge_player_ability(manager, p, club, 1, level as i16) < threshold {
        // below the manager's threshold
        let wage_left = || {
            let years = (c.contract_expires_year() as u16 as i32).wrapping_sub(read_u16(G_CURRENT_DATE + 2) as i32);
            let days = years.wrapping_mul(365).wrapping_add(c.contract_expires() as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
            days.wrapping_mul(c.wage()) / 30
        };
        let value = p.value() as i32;
        let surplus = (c.contract_type() & 0x3f == 2
            && (exe::date_days_since(G_CURRENT_DATE, read_u32(p.addr() + staff::DATE_JOINED_CLUB), read_u32(p.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR))
                <= p.professionalism() as i32 + p.pressure() as i32 + 450
                || exe::date_days_since(c.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) >= 450))
            || value >= rep.wrapping_mul(50)
            || value >= wage_left();
        if surplus {
            if exe::database_count_club_squad(club, 0, 0) as i8 as i32 > rep / 500 + 20 { // mkb2: the squad size that allows listing
                if s.reset {
                    c.set_happiness(-100);
                    c.set_unhappy_flag(0);
                    c.set_matches_left_out(0);
                    exe::contract_manager_lower_match_morale(cm, p, club, c);
                }
                if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0
                    && exe::match_man_judge_player_ability(manager, p, club, 2, level as i16) < cut as i16
                    && (age_i > p.player().goalkeeper() as i32 / 5 + 21 || p.player().current_reputation() as i32 > rep / 2)
                {
                    covered!("mkb_2 transfer listed");
                    const TRANSFER_LIST_PLAYER: usize = 0x842320; // thiscall, ret 8
                    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TRANSFER_LIST_PLAYER);
                    f(s.this, p.addr(), !s.reset as u32);
                }
            }
        } else if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0
            && !s.reset
            && exe::transfer_manager_get_offers_for_player(tm, p, 0usize, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0usize) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, p) == 0
            && exe::database_count_club_squad(club, 0, 0) as i8 as i32 > rep / 500 + 25 // mkb2: the squad size that frees a contract
        {
            covered!("mkb_2 free transfer");
            const QUEUE_FREE_TRANSFER: usize = 0x8c16b0; // thiscall, ret 0xc
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(QUEUE_FREE_TRANSFER);
            f(tm, p.addr(), 0, 0xffff_ffff);
        }
        assess_to_reserves_unless_star(s, p);
        return;
    }
    // at or above the threshold (or not a player)
    if s.on_holiday != 0 {
        return;
    }
    if s.reset {
        c.set_happiness(-50);
        c.set_unhappy_flag(0);
        c.set_matches_left_out(0);
        exe::contract_manager_lower_match_morale(cm, p, club, c);
    }
    assess_to_reserves_unless_star(s, p);
    if s.partner == 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        let h = c.happiness();
        if h >= 0 || p.job_for_club() != 0xb {
            return;
        }
        let limit = if p.player().goalkeeper() == 20 {
            if age <= 19 {
                return;
            }
            age_i * 50 / 3 - 433
        } else {
            if age <= 17 {
                return;
            }
            age_i * 50 / 3 - 400 // mkb2: the unhappiness that puts a player on the loan list
        };
        if h as i32 > limit {
            return;
        }
        covered!("mkb_2 loan listed");
        c.set_transfer_status(c.transfer_status() | 2);
        if age < 24 {
            c.set_squad_status((c.squad_status() & 0xf) | 0x60);
        }
    } else {
        let too_old: i8 = if p.player().goalkeeper() == 20 { 27 } else { 23 };
        if age >= too_old {
            return;
        }
        let j = exe::match_man_judge_player_ability(manager, p, club, 1, ftol(level as i16 as f64 * read_f64(rdata::F64_0_75)) as i16);
        let status = if j as i32 > threshold as i32 + 20 { 0x50 } else { 0x60 };
        c.set_squad_status((c.squad_status() & 0xf) | status);
    }
}

/// The happiness of those picked for the slots of status 2 and 3 (0x83de85..0x83e185), with no manager on holiday:
/// for a picked player of the club with a contract, his score for the slot's position `own` against the best score of
/// the others (the other picks of that list, the picks of 4 and 5 and the rest) `best`: v = ftol((100 − min(best,
/// own) / max(own, 1) × 100) × 0.5) as a byte, then v + 20 − v/5; with `reset` happiness + v (within −100..100),
/// matches left out v/10 (at most 100) and the unhappy flag 0; above 20 the squad status 1 (v ≥ 30 and own above the
/// target rating) or 2. The original calls `get_contract_for_club` with a null person too.
unsafe fn assess_chosen_happiness(s: &mut SquadAssessment) {
    if s.n_a <= 0 || s.on_holiday != 0 {
        return;
    }
    let cm = read_usize(G_CONTRACT_MANAGER);
    for i in 0..s.n_a as usize {
        let p = Staff(s.chosen_a[i]);
        if !p.is_null() && p.club_job() != s.club {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, s.club));
        if c.is_null() || p.is_null() {
            continue;
        }
        let mut flags = s.flags | 0xc0;
        if exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) != 0 {
            flags |= 0x810000;
        }
        let pos = s.pos_a[i] as u32;
        let own = assess_score(s, p.addr(), pos, flags);
        s.rating = own;
        let mut best = 0i32;
        let mut consider = |q: usize| {
            if q != 0 {
                let r = assess_score(s, q, pos, flags);
                if best <= r {
                    best = r;
                }
            }
        };
        for k in 0..s.n_a as usize {
            if k != i {
                consider(s.chosen_a[k]);
            }
        }
        for k in 0..s.n_b.max(0) as usize {
            consider(s.chosen_b[k]);
        }
        for k in 0..s.n_c.max(0) as usize {
            consider(s.chosen_c[k]);
        }
        for k in 0..s.n_rest.max(0) as usize {
            consider(s.rest[k]);
        }
        let m = if best > own { own } else { best };
        let d = if own < 1 { 1 } else { own };
        let x = ftol((read_f32(rdata::F32_100) as f64 - m as f64 / d as f64 * read_f64(rdata::F64_100)) * read_f32(rdata::F32_0_5) as f64) as i8;
        let v = x.wrapping_add(20i8.wrapping_sub((x as i32 / 5) as i8));
        if s.reset {
            c.set_happiness(assess_happiness_plus(c.happiness(), v));
            let q = (v as i32 / 10) as i8;
            c.set_matches_left_out((if q > 100 { 100 } else { q }) as u8);
            c.set_unhappy_flag(0);
        }
        if v > 20 {
            let status = if v >= 30 && own > s.target { 0x10 } else { 0x20 };
            c.set_squad_status((c.squad_status() & 0xf) | status);
        }
    }
}

/// From the date the picked player joined the club, if after the manager's, the slot's date is that + 120 days
/// (only with `reset` and a manager).
unsafe fn assess_slot_date(s: &SquadAssessment, slot: ShortlistSlot, p: Staff) {
    let manager = s.club.manager();
    if !s.reset || manager.is_null() {
        return;
    }
    let (year, manager_year) = (p.date_joined_club_year(), manager.date_joined_club_year());
    if year > manager_year || (year == manager_year && read_i16(p.addr() + staff::DATE_JOINED_CLUB) > read_i16(manager.addr() + staff::DATE_JOINED_CLUB)) {
        let mut later = [0u32; 2];
        let mut scratch = [0u32; 2];
        let d = exe::date_add_days(p.addr() + staff::DATE_JOINED_CLUB, later.as_mut_ptr() as usize, 0x78);
        exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, d);
    }
}

/// The picks fill the free slots in order (0x83e185..0x83e66f); see `assess_club_squad`.
unsafe fn assess_fill_slots(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let club = s.club;
    let mut k: usize = 0;
    for list in 0..3u8 {
        let (n, chosen) = match list {
            0 => (s.n_a, s.chosen_a),
            1 => (s.n_b, s.chosen_b),
            _ => (s.n_c, s.chosen_c),
        };
        for &p in chosen.iter().take(n.max(0) as usize) {
            while assess_slot(s.entry, k).target_staff_id() as i32 > -1 {
                k += 1;
            }
            let slot = assess_slot(s.entry, k);
            let p = Staff(p);
            if p.is_null() {
                slot.set_replacement_staff_id(0xffff_ffff);
                slot.set_target_staff_id(usize::MAX);
                k += 1;
                continue;
            }
            slot.set_replacement_staff_id(read_u32(p.addr()));
            let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, club));
            if s.on_holiday == 0 && !c.is_null() {
                let st = c.squad_status() & 0xf0;
                if list != 0 || (st != 0x10 && st != 0x20) {
                    c.set_squad_status(((slot.flags() & 0xfe) << 3) | (c.squad_status() & 0xf));
                }
            }
            assess_slot_date(s, slot, p);
            k += 1;
            match list {
                0 => {}
                1 => {
                    if (s.partner != 0 || exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0) && k == 25 && p != club.manager() && !s.big_club {
                        assess_move_to_reserves(p, club);
                    }
                    let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p.addr()));
                    let sheet = p.player();
                    // Defect of the original: WORK_RATE is a signed byte and 3250 is out of its range, so this test
                    // is always true (probably WORLD_REPUTATION, a word, was meant); kept as the original does it.
                    if fatigue < -1000 && sheet.current_reputation() < 4500 && (sheet.work_rate() as i32) < 3250 && !c.is_null() && c.happiness() < -50 {
                        covered!("mkb_2 tired reserve loan listed");
                        c.set_transfer_status(c.transfer_status() | 2);
                    }
                }
                _ => {
                    if s.partner == 0 && p != club.manager() {
                        assess_move_to_reserves(p, club);
                    }
                    let r = exe::utils_random(300);
                    let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p.addr())) as i32;
                    if fatigue < -500 - r && !c.is_null() && c.happiness() < 0 {
                        covered!("mkb_2 tired spare loan listed");
                        c.set_transfer_status(c.transfer_status() | 2);
                    }
                }
            }
        }
    }
}

/// The player slots without a target are priced (0x83e66f..0x83ebfb); see `assess_club_squad`.
unsafe fn assess_price_slots(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let club = s.club;
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        if slot.target_staff_id() as i32 > -1 || read_u8(slot.addr()) != 0xb {
            continue;
        }
        let mut price = if s.target >= 36000 { 36000 } else { s.target };
        if slot.position() == 1 && slot.replacement_staff_id() as i32 == -1 {
            price = ftol(price as f64 * read_f64(rdata::F64_0_75));
        }
        let replacement = slot.replacement_staff_id() as i32;
        if replacement <= -1 {
            slot.set_valuation(price);
            slot.set_target_staff_id(usize::MAX);
            slot.set_max_price(0);
            let (p, r) = (price as f64, club.reputation() as f64);
            let v = match (slot.flags() >> 1) & 0xf {
                1 => p * read_f64(rdata::F64_2_5) + r,
                2 => (p + p) + r,
                3 => p * read_f64(rdata::F64_1_8) + r,
                4 | 5 => p * read_f64(rdata::F64_0_8),
                _ => p * read_f64(rdata::F64_0_5),
            };
            slot.set_price(ftol(v));
            if read_u8(slot.addr() + shortlist_slot::POSITION) & 1 != 0 {
                slot.set_price(slot.price().wrapping_mul(3)); // a goalkeeper slot
            }
            continue;
        }
        let person = staff_at(replacement as isize);
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, person, club));
        let pos = slot.position() as u16 as u32;
        match (slot.flags() >> 1) & 0xf {
            2 | 3 => {
                if !c.is_null() && s.reset {
                    c.set_happiness(assess_happiness_plus(c.happiness(), 50));
                    c.set_matches_left_out(10);
                    c.set_unhappy_flag(0);
                }
                assess_move_to_first_team(person, club);
                let flags = if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 { s.flags | 0x8100c0 } else { s.flags | 0xc0 };
                s.rating = assess_score(s, person.addr(), pos, flags);
                if s.days_since_start < 200 {
                    s.rating = ftol((read_f64(rdata::F64_1_25) - s.days_since_start as f64 * read_f64(0x96f888)) * s.rating as f64); // f64 0.00125
                }
            }
            4 => {
                assess_move_to_first_team(person, club);
                s.rating = assess_score(s, person.addr(), pos, s.flags | 0x1c0);
            }
            5 => {
                if person != club.manager() {
                    assess_to_reserves_unless_star(s, person);
                }
                s.rating = assess_score(s, person.addr(), pos, s.flags | 0x100);
            }
            _ => {} // the score of the previous slot or player (stale +0x38 in the original)
        }
        if s.rating < -10000 {
            s.rating = -10000;
        } else if s.rating > 60000 {
            s.rating = 60000;
        }
        let r = s.rating;
        let unhappy = || !c.is_null() && c.grievances() != 0 && (c.grievances() & 0x100 != 0 || c.grievances() & 1 != 0 || exe::staff_contract_days_to_end_date(c) < 365);
        if r >= price && !unhappy() {
            slot.set_target_staff_id(read_u32(person.addr()) as usize); // he keeps the place
            continue;
        }
        covered!("mkb_2 slot priced");
        let delta = ftol((read_f64(rdata::F64_1) - r as f64 / price as f64) * read_f64(0x96f880)) as i8; // f64 -100.0
        let st = slot.flags() & 0x1e;
        if !c.is_null() {
            if s.reset {
                c.set_happiness(assess_happiness_plus(c.happiness(), delta));
            }
            if st == 4 || st == 6 {
                if c.happiness() < 25 && s.reset {
                    c.set_happiness(25);
                }
                slot.set_target_staff_id(usize::MAX);
            } else if c.happiness() < 0 && s.reset {
                c.set_happiness(0);
            }
            if s.on_holiday == 0 && slot.flags() & 0x1e == 0xa {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            }
        } else if st == 4 || st == 6 {
            slot.set_target_staff_id(usize::MAX);
        }
        slot.set_valuation(price);
        slot.set_max_price(0);
        let v = if price > 1000 { price } else { 1000 };
        let x = read_f64(rdata::F64_1_25) - r as f64 / v as f64;
        let f = if x > read_f64(rdata::F64_1) {
            read_f64(rdata::F64_1)
        } else if x < read_f64(rdata::F64_0_5) {
            read_f64(rdata::F64_0_5)
        } else {
            x
        };
        let (p, rf, rr) = (price as f64, r as f64, club.reputation() as f64);
        let price_left = match (slot.flags() >> 1) & 0xf {
            1 => (rr * f + p * read_f64(rdata::F64_2_5)) - rf * read_f64(rdata::F64_2_5),
            2 => (rr * f + (p + p)) - (rf + rf),
            3 => (rr * f + p * read_f64(rdata::F64_1_8)) - rf * read_f64(rdata::F64_1_8),
            4 | 5 => p * read_f64(rdata::F64_0_8) - rf * read_f64(rdata::F64_0_8),
            _ => p * read_f64(rdata::F64_0_5) - rf * read_f64(rdata::F64_0_5),
        };
        slot.set_price(ftol(price_left));
    }
}

/// The staff slots (0x83ebfb..0x83f321); see `assess_club_squad`.
unsafe fn assess_staff_slots(s: &mut SquadAssessment) {
    let club = s.club;
    let mut staff = [0usize; 16];
    let mut n: i8 = 0;
    let mut add = |q: usize| {
        if q != 0 && exe::shortlist_manager_find_club_shortlist_entry(s.this, club, q, 1, 0) == 0 {
            staff[n as usize] = q;
            n += 1;
        }
    };
    add(club.assistant_manager());
    for k in 0..5 {
        add(read_usize(club.addr() + club::COACHES + 4 * k));
    }
    for k in 0..3 {
        add(read_usize(club.addr() + club::PHYSIOS + 4 * k));
    }
    for k in 0..7 {
        add(read_usize(club.addr() + club::SCOUTS + 4 * k));
    }
    let threshold = if s.target / 5 >= 5000 { 5000 } else { s.target / 5 };
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        if read_u8(slot.addr()) == 0xb {
            continue;
        }
        slot.set_flags(slot.flags() & 0xfe);
        slot.set_max_price(0);
        slot.set_price(ftol(threshold as f64 * read_f64(rdata::F64_2_5)));
        if slot.offer_ref() != -1 {
            continue;
        }
        slot.set_valuation(threshold);
        let jobs: [u8; 2] = match read_i8(slot.addr()) {
            6 => [6, 0xd],
            7 => [7, 0xe],
            8 => [8, 0xf],
            9 => [9, 9],
            10 => [10, 10],
            _ => continue,
        };
        let Some(i) = (0..n.max(0) as usize).find(|&i| jobs.contains(&Staff(staff[i]).job_for_club())) else {
            continue;
        };
        let person = Staff(staff[i]);
        slot.set_replacement_staff_id(read_u32(person.addr()));
        let manager = if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 { s.stand_in } else { club.manager().addr() };
        let score = exe::score_staff_for_job(read_usize(G_MANAGER_MANAGER), person, club, manager as u32, read_u8(slot.addr()) as u32);
        if score >= threshold {
            slot.set_target_staff_id(read_u32(person.addr()) as usize);
            slot.set_price(0);
        } else if s.may_sack && exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(person.addr()), read_u32(club.addr())) == 0 {
            covered!("mkb_2 staff sacked");
            slot.set_replacement_staff_id(0xffff_ffff);
            slot.set_target_staff_id(usize::MAX);
            exe::manager_manager_remove_staff_from_club(read_usize(G_MANAGER_MANAGER), person, club);
        } else {
            slot.set_target_staff_id(usize::MAX);
            slot.set_price(ftol((threshold - score) as f64 * read_f64(rdata::F64_2_5)));
        }
        n -= 1;
        staff[i] = staff[n as usize];
    }
}

/// The AI's assessment of a club's squad, done when its shortlist is rebuilt: it clears and sets the squad statuses,
/// sends players to the first team or the reserves, puts players on the transfer list, the loan list or the free
/// transfer queue, fills and prices the shortlist slots and may sack idle staff.
///
/// `Shortlist_Manager::assess_club_squad(entry, club, daily, reset)`, from `update_club_shortlist` (0x83a730; daily 1,
/// reset 0) when the club's list is redone, and with daily 0, reset 1 from `create_initial_shortlists` (0x83a1b0, at
/// career creation) and `rebuild_club_shortlist` (0x84abe0, a new manager). `entry` is the club's 9-byte record in
/// `shortlist_manager_fields::CLUB_SHORTLISTS` (+0 its 40 slots of 0x25 bytes, +4 `club_shortlist::ACTIVE`, +7 the
/// formation, +8 `club_shortlist::DAYS`).
///
/// 1. DAYS = 0; reset is forced when slot 0's `shortlist_slot::INDEX` is −1. It leaves with ACTIVE 0 for a national
///    team, a club without a nation, a human club when `daily`, `club::PROFESSIONAL_STATUS` 3, or a club whose linked B
///    team lookup (`exe::club_get_linked_b_team`) succeeds with no partner. With bids out
///    (`exe::shortlist_manager_count_slots_with_bids`) and ACTIVE 2 it leaves untouched. With no bids: ACTIVE 2 becomes
///    1 with reset, and idle staff may be sacked at an AI club; a club of an unloaded league (no `.league_selected()`,
///    no division or the division not in `G_COMP_OBJECT_TABLE`) with reputation below 4000 switches its list off
///    (ACTIVE 0) when random(reputation + 1000) < random(5000). Then ACTIVE = 1.
/// 2. The continental entry (`.continental_comp_id()`, or the club state's when none, or the Inter-Toto at a club of
///    reputation ≥ 5000), the target rating and its base (`player_search_get_club_target_rating`), the club's target
///    reputation (`update_club_target_reputation`) and, at a human club, the stand-in manager and the holiday state.
/// 3. With reset the slots are rebuilt (`assess_set_up_slots`); otherwise they are sorted by index (qsort, 0x83b6c0).
/// 4. `assess_release_slots`, `assess_sort_squad` and `assess_pick_squad` (`match_man_assign_players_to_positions`).
/// 5. **The rest** (50 entries): a player on loan here from another club goes to the reserves (except a star with a B
///    team); one of the club with a contract, not queued for a free transfer (with reset, queued ones get happiness
///    −100): the threshold is ftol((age × 0.13 − 1.98) × base × 0.02), the cut base / 68 − 35 and his level
///    ftol(current reputation × 0.25 + potential ability × 12.5 + current ability × 12.5 + club reputation × 0.25). A
///    player the manager judges below the threshold (`match_man_judge_player_ability` mode 1) is surplus if his
///    contract is of type 2 and he joined less than professionalism + pressure + 450 days ago or it ends in 450 days or
///    more, or `staff::VALUE` ≥ reputation × 50, or the value ≥ the wages left (days to the end × wage / 30). A surplus
///    player, with the squad (`database_count_club_squad`) above reputation / 500 + 20: with reset happiness −100 and
///    `contract_manager_lower_match_morale`; at an AI club, judged (mode 2) below the cut and older than goalkeeper / 5
///    + 21 or with current reputation above half the club's, **transfer listed** (`transfer_list_player`, news only
///    without reset). Otherwise, at an AI club without reset, no offers or exchange offers, not queued, and the squad
///    above reputation / 500 + 25: **free transfer queue** (`queue_free_transfer`). Both go to the reserves (except a
///    star with a B team). A player at or above the threshold, with no manager on holiday: with reset happiness −50
///    and lower morale; reserves (same exception); at an AI club without a B team, a player with negative happiness on
///    the **loan list** (`transfer_status::LOAN_LISTED`, status 6 under 24) when a goalkeeper rated 20 aged 20 or more
///    has happiness ≤ age × 50 / 3 − 433, or another player aged 18 or more ≤ age × 50 / 3 − 400; with a B team or at
///    a human club, under 27 (goalkeeper 20) or 23, the status 5 when judged with three quarters of his level above
///    the threshold + 20, else 6.
/// 6. `assess_chosen_happiness`, then the picks fill the free slots in order (`assess_fill_slots`): the replacement
///    is the pick, his contract takes the slot's status (list 2/3 not over statuses 1 and 2) when no manager is on
///    holiday, the slot date is set from his arrival (`assess_slot_date`). Status 4: at slot 24, with a B team or at a
///    human club and not a big club, the pick (not the manager) goes to the reserves; **loan list** when fatigue <
///    −1000, current reputation < 4500, `player_sheet::WORK_RATE` < 3250 and happiness < −50. **Defect of the
///    original:** WORK_RATE is a signed byte, so `< 3250` is always true (probably `WORLD_REPUTATION` was meant); kept.
///    Status 5: to the reserves without a B team (not the manager); **loan list** when fatigue < −500 − random(300)
///    and happiness < 0.
/// 7. `assess_price_slots`: each player slot without a target is priced from p = min(target rating, 36000) (× 0.75
///    for a goalkeeper slot without a replacement). Without a replacement: valuation p and price p × 2.5 + reputation,
///    2p + reputation, p × 1.8 + reputation, p × 0.8 or p × 0.5 by status 1, 2, 3, 4/5, other; × 3 for a goalkeeper
///    slot. With one: by the status he goes to the first team (2, 3 and 4; with reset happiness + 50, 10 matches left
///    out) or to the reserves (5) and is scored for the position (in the first 200 days × (1.25 − days × 0.00125)),
///    within −10000..60000; a score ≥ p with no grievances 0x100 or 1 and more than 364 days of contract keeps him as
///    the target. Otherwise his happiness moves by ftol((1 − score / p) × −100) (with reset; statuses 2 and 3 at least
///    25 and the slot loses him, the others at least 0; status 5 gets squad status 6), the valuation is p and the
///    price (reputation × f + p × k) − score × k for k 2.5, 2, 1.8 (statuses 1..3, f = 1.25 − score / max(p, 1000)
///    within 0.5..1), or p × k − score × k with k 0.8 (4/5) or 0.5.
/// 8. `assess_staff_slots`: the assistant, coaches, physios and scouts not on the club's list are matched to the staff
///    slots by job (6 or 13, 7 or 14, 8 or 15, 9, 10); threshold min(target / 5, 5000): every staff slot loses bit 0
///    and the ceiling and gets the price threshold × 2.5; without a bid the valuation is the threshold and the first
///    person of the job is scored (`score_staff_for_job`): at or above it he is the target (price 0); below it, when
///    staff may be sacked and he has no contract offer (`contract_manager_find_contract_offer`), he is **sacked**
///    (`manager_manager_remove_staff_from_club`); otherwise price (threshold − score) × 2.5.
///
/// The floating point is in f64 (the game's x87 at 53 bits) in the original's order of operations.
/// Original: `Shortlist_Manager::assess_club_squad` (0x83cb40, shortlist_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn assess_club_squad(this: usize, entry: usize, club: usize, daily: u32, reset: u32) {
    let club = Club(club);
    let mut reset = reset as u8 != 0;
    let days_since_start = exe::date_days_since(G_CURRENT_DATE, read_u32(G_GAME_START_DATE), read_u32(G_GAME_START_DATE_FLAGS));
    let flags: u32 = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { 0x800 } else { 0x1802 };
    if assess_slot(entry, 0).index() == -1 {
        reset = true;
    }
    let Some((partner, may_sack)) = assess_start(this, entry, club, daily, &mut reset) else {
        return;
    };
    let mut s = SquadAssessment {
        this,
        entry,
        club,
        reset,
        flags,
        partner,
        comp: 0,
        target: 0,
        target_base: 0,
        days_since_start,
        stand_in: 0,
        on_holiday: 0,
        may_sack,
        big_club: false,
        rating: 0, // uninitialised in the original until the first score
        pos_a: [0; 50],
        n_a: 0,
        pos_b: [0; 50],
        n_b: 0,
        pos_c: [0; 50],
        n_c: 0,
        chosen_a: [0; 50],
        chosen_b: [0; 50],
        chosen_c: [0; 50],
        rest: [0; 100],
        n_rest: 0,
        elsewhere: [0; 100],
        n_elsewhere: 0,
        old: [0; 50],
        n_old: 0,
    };
    let mut comp_id = club.continental_comp_id() as i32;
    if comp_id <= -1 || (comp_id == read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) && club.reputation() >= 5000) {
        comp_id = read_i32(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(6)));
    }
    if comp_id > -1 {
        s.comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b));
    }
    const TARGET_RATING: usize = 0x7bca90; // player_search_get_club_target_rating, cdecl
    let target_rating: unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32 = core::mem::transmute(TARGET_RATING);
    s.target = target_rating(club.addr(), &mut s.target_base, core::ptr::null());
    const TARGET_REPUTATION: usize = 0x83bf40; // update_club_target_reputation, thiscall, ret 4
    let target_reputation: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(TARGET_REPUTATION);
    target_reputation(this, club.addr());
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        s.stand_in = exe::human_manager_get_stand_in_manager(club);
        let manager = exe::database_get_club_manager(club);
        s.on_holiday = exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), manager);
    }
    if s.reset {
        assess_set_up_slots(&mut s);
    } else {
        exe::crt_qsort(read_usize(entry), 0x28, 0x25, 0x83b6c0usize); // shortlist_compare_slots_by_index
    }
    assess_release_slots(&mut s);
    assess_sort_squad(&mut s);
    assess_pick_squad(&mut s);
    for k in 0..50 {
        let p = Staff(s.rest[k]);
        if !p.is_null() {
            assess_rest_player(&mut s, p);
        }
    }
    assess_chosen_happiness(&mut s);
    assess_fill_slots(&mut s);
    assess_price_slots(&mut s);
    assess_staff_slots(&mut s);
}

/// Comparer for `assess_club_squad`: the original only (random draws, moves people between first team and reserves,
/// lists players, may sack staff: it cannot run twice), and the plan is checked on exit: DAYS 0; ACTIVE 0 or 1, or 2
/// only if it was 2; after a full run (ACTIVE 1) every staff slot has bit 0 clear and no ceiling, and with `reset`
/// every slot has its own index.
pub(crate) unsafe extern "thiscall" fn compare_assess_club_squad(this: usize, entry: usize, club: usize, daily: u32, reset: u32) {
    let before = read_u8(entry + club_shortlist::ACTIVE);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%)(this, entry, club, daily, reset);
    let active = read_u8(entry + club_shortlist::ACTIVE);
    let mut ok = read_u8(entry + club_shortlist::DAYS) == 0 && (active <= 1 || (active == 2 && before == 2));
    let mut bad_slot = usize::MAX;
    if active == 1 {
        for k in 0..40usize {
            let slot = assess_slot(entry, k);
            let staff_ok = read_u8(slot.addr()) == 0xb || (slot.flags() & 1 == 0 && slot.max_price() == 0);
            let index_ok = reset as u8 == 0 || slot.index() as usize == k;
            if !(staff_ok && index_ok) {
                ok = false;
                bad_slot = k;
                break;
            }
        }
    }
    SCOREBOARD.record(ok, || {
        format!("assess_club_squad(club {}, daily {}, reset {}) ACTIVE {} -> {}, DAYS {}, slot {:?}", read_i32(club), daily as u8, reset as u8, before, active, read_u8(entry + club_shortlist::DAYS), if bad_slot == usize::MAX { None } else { Some(bad_slot) })
    });
}
