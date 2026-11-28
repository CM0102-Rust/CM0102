// --- r7_08_3: a player's squad status set from the transfer screens (transfer_manager.cpp) ---

/// A contract's happiness or morale moved by `d` within −100..100 (the original's byte arithmetic).
fn squad_mood(v: i8, d: i32) -> i8 {
    let s = v as i32 + d;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        s as i8
    }
}

/// The morale change of a new squad status (0x8ce232..0x8ce52a). From no status (high nibble 0): 1 +100, 2 +50, 3 +25,
/// or with `flags` & 3 −100 and `out` = 1; 4 nothing, or with `flags` & 3 −100 and `out` = 1; 5 nothing, or with
/// `flags` & 1 −50 and `out` = 1; 6 −20; 7 −100. From a better status (a lower number): (old − new) × 25, at least
/// −125, or −50 from 3 to 4.
unsafe fn squad_status_morale(c: Contract, status: i8, flags: u32, out: *mut u32) {
    let old = c.squad_status();
    let add = |d: i32| c.set_morale(squad_mood(c.morale(), d));
    let refuse = |d: i32| {
        add(d);
        *out = 1;
    };
    if old & 0xf0 == 0 {
        let asked = flags & 3 != 0;
        match status {
            1 => if asked { refuse(-100) } else { add(100) },
            2 => if asked { refuse(-100) } else { add(50) },
            3 => if asked { refuse(-100) } else { add(25) }, // r7_08_3: rotation from no status
            4 => if asked { refuse(-100) },
            5 => if flags & 1 != 0 { refuse(-50) },
            6 => add(-20),
            7 => add(-100),
            _ => {}
        }
        return;
    }
    let was = (old >> 4) as i8;
    if was >= status {
        return;
    }
    let d = if old & 0xf0 == 0x30 && status == 4 { -50 } else { ((was as i16 - status as i16) * 25).max(-125) as i32 }; // r7_08_3: 25 per place
    add(d);
}

/// The age under which a squad member resents being dropped (0x8ce5b3..0x8ce667): 32, 31 for a loyalty above
/// random(3) + 15 or 33 below random(3) + 5; one less for a professionalism above random(3) + 15, one more below
/// random(3) + 5; one more for a goalkeeping rating above 15, one less for an attacking one above 15; an injury of more
/// than 180 days −10, more than 90 days −7 (professionalism above 5) and more than 30 days −2.
unsafe fn squad_status_age_limit(p: Staff) -> i8 {
    let draw = |n: i32| exe::utils_random(n);
    let mut limit: u8 = 0x20;
    if p.loyalty() as i32 > draw(3) + 15 {
        limit = 0x1f;
    } else if (p.loyalty() as i32) < draw(3) + 5 {
        limit = 0x21;
    }
    if p.professionalism() as i32 > draw(3) + 15 {
        limit = limit.wrapping_sub(1);
    } else if (p.professionalism() as i32) < draw(3) + 5 {
        limit = limit.wrapping_add(1);
    }
    let sheet = p.player();
    if !sheet.is_null() {
        if sheet.goalkeeper() > 15 {
            limit = limit.wrapping_add(1);
        }
        if sheet.attacker() > 15 {
            limit = limit.wrapping_sub(1);
        }
    }
    let injury = exe::injury_get_player_injury(read_usize(G_INJURY), p);
    if injury != 0 {
        let days = read_i16(injury + 0x2c); // the injury's days left
        if days > 180 {
            limit = limit.wrapping_sub(8);
        } else if days > 90 && p.professionalism() > 5 {
            limit = limit.wrapping_sub(5);
        }
        if days > 30 {
            limit = limit.wrapping_sub(2);
        }
    }
    limit as i8
}

/// The player's reaction to the new status once the game is a week old (0x8ce54a..0x8ce896). "Upset" is
/// `exe::staff_contract_apply_event_flags` 0x200; "by character" means random(professionalism) or random(loyalty) gives 0.
unsafe fn squad_status_reaction(p: Staff, club: Club, c: Contract, status: i8) {
    let draw = |n: i32| exe::utils_random(n);
    let upset = || exe::staff_contract_apply_event_flags(c, 0x200, 0);
    let by_character = || draw(p.professionalism() as i32) == 0 || draw(p.loyalty() as i32) == 0;
    if status == 1 {
        if c.grievances() & 0x4000 != 0 {
            exe::staff_contract_clear_grievances(c, 0x4000);
            if by_character() {
                upset();
            }
        }
        return;
    }
    let old = c.squad_status();
    if old & 0xf0 == 0 || (old >> 4) as i8 >= status {
        return;
    }
    let limit = squad_status_age_limit(p);
    if old & 0xf0 == 0x10 && (p.age() as i8) < limit {
        if status != 2 && (by_character() || draw(5) == 0) {
            covered!("r7_08_3 key player unhappy with his role");
            const NEWS_PLAYER_UNHAPPY_WITH_ROLE: usize = 0x5882d0; // news_post_player_unhappy_with_role, cdecl(player, club)
            let news: unsafe extern "C" fn(usize, usize) = core::mem::transmute(NEWS_PLAYER_UNHAPPY_WITH_ROLE);
            news(p.addr(), club.addr());
            exe::staff_contract_apply_event_flags(c, 0x4000, 0);
            c.set_morale(squad_mood(c.morale(), -5));
            return;
        }
        // status 2, or a player who keeps quiet, falls into the same test (the original's code is shared)
        if draw(5) == 0 && draw(p.ambition() as i32) != 0 {
            upset();
        }
        return;
    }
    if status == 3 && old & 0xf0 == 0x20 && club.reputation() >= 0x1676 {
        return;
    }
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(c.addr() + contract::DATE_STARTED), read_u32(c.addr() + contract::DATE_STARTED_LEAP_YEAR));
    if days < (p.ambition() as i32 + 6) * 5 {
        upset(); // r7_08_3: a recent signing dropped
        return;
    }
    match exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p, c) as i8 {
        1 => {
            if by_character() || draw(3) == 0 {
                upset();
                c.set_morale(squad_mood(c.morale(), -10));
            }
        }
        2 => {
            if draw(10) != 0 && by_character() {
                upset();
            }
        }
        3 | 4 => {
            if (p.age() as i8) < limit && draw(5) != 0 && p.ambition() > 10 && by_character() {
                upset();
            }
        }
        5 => {
            if p.ambition() > 15 && draw(3) != 0 && by_character() {
                upset();
            }
        }
        _ => {}
    }
}

/// Sets a player's squad status at his club from the transfer screens, with his asking price, and moves his happiness
/// and morale; a dropped key player may be upset or post an "unhappy with his role" story; then the AI shortlists are
/// refreshed for him.
///
/// `TRANSFER_MANAGER::set_player_squad_status(p, club, value, out, status, flags, unlist)`, from the transfer screens
/// (0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc). A club in crisis (`finance::CRISIS_STATE` 1 of
/// `exe::financetable_get_club_finance`) is noted. The player's contract at the club
/// (`exe::contract_manager_get_contract_for_club`); without one, the "Error" box (transfer_manager.cpp line 0x2f0c) and
/// nothing else. A player (`.player()`) under a type 2 contract gets `value` as his `.value()`, at a club in crisis only
/// when it is at most what the human club on screen would pay (`exe::contract_manager_calc_player_value` for the
/// manager's club, 0, −1, 1).
///
/// Status −1 changes nothing but the end. Status 7 (not wanted) takes 100 off the contract's happiness unless he already
/// had it. Then the morale (`squad_status_morale`), and once the game is a week old
/// (`exe::date_days_since` from `G_GAME_START_DATE` ≥ 7) the reaction (`squad_status_reaction`); the high nibble of
/// `contract::SQUAD_STATUS` becomes the status. At the end, `unlist` outside a club in crisis clears bit 2 of
/// `contract::TRANSFER_STATUS`, and the shortlists stop and restart (`exe::shortlist_manager_suspend_clubs_shortlisting_player`,
/// `exe::shortlist_manager_add_player_to_club_shortlists`, both with 0).
/// Original: `TRANSFER_MANAGER::set_player_squad_status` (0x8ce0e0, transfer_manager.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn set_player_squad_status(this: usize, p: usize, club: usize, value: i32, out: *mut u32, status: u32, flags: u32, unlist: u32) {
    let p = Staff(p);
    let club = Club(club);
    let crisis = !club.is_null() && read_u8(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) + finance::CRISIS_STATE) == 1;
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
    if c.is_null() {
        show_error_box(0xa8d514, 0x2f0c);
        return;
    }
    if c.contract_type() & 0x3f == 2 && !p.player().is_null() {
        if !crisis || value <= exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, current_screen_manager().club_job(), 0, -1, 1) {
            p.set_value(value as usize);
        }
    }
    let status = status as i8;
    if status != -1 {
        if status == 7 && c.squad_status() & 0xf0 != 0x70 {
            c.set_happiness(squad_mood(c.happiness(), -100)); // r7_08_3: not wanted
        }
        squad_status_morale(c, status, flags, out);
        if exe::date_days_since(G_CURRENT_DATE, read_u32(G_GAME_START_DATE), read_u32(G_GAME_START_DATE_FLAGS)) >= 7 {
            squad_status_reaction(p, club, c, status);
        }
        c.set_squad_status(((status as u8) << 4) | (c.squad_status() & 0xf));
    }
    if unlist != 0 && !crisis {
        c.set_transfer_status(c.transfer_status() & 0xfb);
    }
    let sm = read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER);
    exe::shortlist_manager_suspend_clubs_shortlisting_player(sm, p, 0);
    exe::shortlist_manager_add_player_to_club_shortlists(sm, p, 0);
}

/// Comparer for `set_player_squad_status`: the original only (it draws random numbers, posts news, flags the contract and
/// redoes the shortlists: it cannot run twice); the plan is checked on exit: with a contract and a status other than
/// −1, the high nibble of `contract::SQUAD_STATUS` is the status.
pub(crate) unsafe extern "thiscall" fn compare_set_player_squad_status(this: usize, p: usize, club: usize, value: i32, out: *mut u32, status: u32, flags: u32, unlist: u32) {
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, *mut u32, u32, u32, u32)>(%%I0%%)(this, p, club, value, out, status, flags, unlist);
    if c.is_null() || status as u8 == 0xff {
        return;
    }
    let got = c.squad_status() & 0xf0;
    SCOREBOARD.record(got == (status as u8) << 4, || format!("set_player_squad_status(person {:#x}) status {} squad status after {:#x}", read_u32(p), status as u8, got));
}
