// ---------- batch %%LEVA%%: appointing a manager to a club (manager_manager.cpp) ----------

/// Exe routines `appoint_manager_to_club` calls that are already Rust but have no `exe::` wrapper (called through the
/// exe, so the comparison mode runs their comparers).
pub(crate) mod appoint_fns {
    pub(crate) const POST_MANAGER_APPOINTMENT_NEWS: usize = 0x6a2e00; // MANAGER_MANAGER::post_manager_appointment_news
    pub(crate) const RESET_BOARD_FOR_NEW_MANAGER: usize = 0x694c80; // MANAGER_MANAGER::reset_board_for_new_manager
    pub(crate) const MANAGER_MANAGER_CPP: usize = 0x9f18a4; // "E:\dev\CM3\cm3 00-01\cm3\code\manager_manager.cpp"
}

/// The bits of the contract's grievance flags (`+0x45`) the new manager clears for each player of the squad, in the
/// original's order.
const NEW_MANAGER_CLEARED_GRIEVANCES: [u32; 7] = [8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000];

/// Takes the person out of the old job before the move: the player data from the old club's squad
/// (`exe::manager_manager_remove_player_from_squad`) and the staff data from its staff
/// (`exe::manager_manager_remove_staff_from_club`), by the old role, and returns the code of the appointment news
/// (1 player, 2 staff, 3 manager). Without the data the role needs, the game's error box with the line of the
/// original (`show_error_at`). `lines` are the error lines of the club or of the national team version.
pub(crate) unsafe fn appoint_leave_old_job(this: usize, p: Staff, old: Club, role: u8, national: bool) -> u32 {
    use appoint_fns::MANAGER_MANAGER_CPP as FILE;
    let staff_out = |line: u32| {
        if !p.non_player().is_null() {
            exe::manager_manager_remove_staff_from_club(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let player_out = |line: u32| {
        if !p.player().is_null() {
            exe::manager_manager_remove_player_from_squad(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let l = |club: u32, nation: u32| if national { nation } else { club };
    match role {
        5 => {
            staff_out(l(0x34c2, 0x3526));
            3
        }
        6 | 8 => {
            staff_out(l(0x34b9, 0x351d));
            2
        }
        11 => {
            player_out(l(0x34a0, 0x3504));
            1
        }
        12 => {
            player_out(l(0x34a9, 0x350d));
            staff_out(l(0x34af, 0x3513));
            if national { 2 } else { 3 } // the player-manager of a club is news 3, of a national team 2
        }
        15 => {
            player_out(l(0x3491, 0x34f5));
            staff_out(l(0x3497, 0x34fb));
            2
        }
        _ => {
            if !p.non_player().is_null() {
                exe::manager_manager_remove_staff_from_club(this, p, old);
            }
            if !p.player().is_null() {
                exe::manager_manager_remove_player_from_squad(this, p, old);
            }
            2
        }
    }
}

/// Moves `manager` to `club` as its manager: vacancies at the old job, the appointment news, the new job, the squad
/// slot of a player-manager, the board reset and the grievances of the squad.
///
/// `MANAGER_MANAGER::appoint_manager_to_club(club, manager)`. The old job: at a national team, the person's national
/// job (`.national_job()`) when he manages one (`.job_for_nation()` 5 or 12); at a club, his club job (`.club_job()`).
/// With an old job: leaving a club he managed (`.job_for_club()` 5 or 12) for a club, the old club gets a vacancy
/// (`exe::manager_manager_add_job_vacancy`) when its nation's league is selected (`nation::LEAGUE_SELECTED`) and its
/// division (`.division()`) has a competition object (`G_COMP_OBJECT_TABLE`); leaving a national team he managed for
/// a national team, when the old one has reputation (`.reputation()`) above 3500 and its nation no continent
/// (`.continent()`); moving to a club, a player-manager or role 15 loses his playing contract
/// (`exe::contract_manager_remove_contract`). The board's `board_record::LAST_REVIEW_DATE`-like date at +0x20 of the
/// club's board row becomes 23/7/1971 (`exe::date_date_549f70`, `exe::date_operator_3d`); without a non-player sheet,
/// the error box (line 0x3480). The old job is left by role (`appoint_leave_old_job`) and the appointment news posted
/// (`post_manager_appointment_news`, code 1..3; without an old job of that kind, with no old club and code 2); at a
/// national team `.job_for_nation()` becomes 5. A human manager: the automatic replies (`exe::news_auto_respond_all`)
/// and, when the club's nation is selected (bit 1 of `nation::LEAGUE_SELECTED`), the old club's nation stops being
/// "the human's" (bit 2 cleared, bit 1 set) unless another human manages in a division of that nation, and the new
/// nation gets bit 2 instead of bit 1. The joining date (`staff::DATE_JOINED_NATION` or `staff::DATE_JOINED_CLUB`)
/// becomes today, the club's `.manager()` the person; at a national team his `.national_job()` the club and job 5; at
/// a club his `.club_job()` the club and job 5, or 12 (player-manager, in the first free of the 50 `.squad()` slots)
/// when his player reputation (`.current_reputation()`) + 750 is above the club's. A human: the scout lists
/// (`exe::scout_manager_sync_scout_lists`) and the training schedules (`exe::training_manager_reassign_squad_schedules`).
/// Then the board reset (`reset_board_for_new_manager`) and, at a club, each squad player's contract
/// (`exe::contract_manager_get_contract`) loses the grievance bits of `NEW_MANAGER_CLEARED_GRIEVANCES`
/// (`exe::staff_contract_clear_grievances`).
/// Original: `MANAGER_MANAGER::appoint_manager_to_club` (0x69ebc0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    let club = Club(club);
    let p = Staff(manager);
    let national = exe::database_is_national_team(club) != 0;
    let manages = |role: u8| role == 5 || role == 12;
    let old = if national && !p.national_job().is_null() && manages(p.job_for_nation()) {
        p.national_job()
    } else if !national && !p.club_job().is_null() {
        p.club_job()
    } else {
        Club(0)
    };
    if !old.is_null() {
        if !national && manages(p.job_for_club()) && old == p.club_job() {
            let n = old.nation();
            let d = old.division();
            if !n.is_null() && n.league_selected() != 0 && !d.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) != 0 {
                covered!("sma_2 vacancy at the old club");
                exe::manager_manager_add_job_vacancy(this, old, p); // sma2: the old club's vacancy
            }
        } else if national && manages(p.job_for_nation()) && old == p.national_job() && old.reputation() > 0xdac && old.nation().continent() == 0 {
            exe::manager_manager_add_job_vacancy(this, old, p);
        }
        if !national && (p.job_for_club() == 15 || p.job_for_club() == 12) {
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), p);
        }
    }
    let mut d = [0u16; 4];
    let mut tmp = [0u8; 8];
    exe::date_date_549f70(&mut d, 0x17, 7, 0x7b3, u32::MAX);
    exe::date_operator_3d(board_row(this, club) + 0x20, tmp.as_mut_ptr() as usize, d.as_ptr() as usize);
    if p.non_player().is_null() {
        show_error_at(appoint_fns::MANAGER_MANAGER_CPP, 0x3480);
    }
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(appoint_fns::POST_MANAGER_APPOINTMENT_NEWS);
    if !national {
        if old.is_null() || old != p.club_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_club(), false);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
    } else {
        if old.is_null() || old != p.national_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_nation(), true);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
        if p.job_for_nation() != 5 {
            p.set_job_for_nation(5);
        }
    }
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    if human {
        covered!("sma_2 human manager");
        exe::news_auto_respond_all(read_usize(G_NEWS), p, old);
        let cn = club.nation();
        if !cn.is_null() && cn.league_selected() & 1 != 0 {
            if !old.is_null() {
                let on = old.nation();
                if !on.is_null() {
                    let count = read_i32(G_STAFF_COUNT);
                    let other = (count - 0x10..count).any(|i| {
                        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE)));
                        let c = s.club_job();
                        s.classification() != 0 && s != p && !c.is_null() && !c.division().is_null() && c.division().nation() == on
                    });
                    if !other {
                        on.set_league_selected(on.league_selected() & 0xfd);
                        old.nation().set_league_selected(old.nation().league_selected() | 1);
                    }
                }
            }
            cn.set_league_selected(cn.league_selected() & 0xfe);
            cn.set_league_selected(cn.league_selected() | 2); // sma2: the human's league mark
        }
    }
    let joined = if national { p.addr() + staff::DATE_JOINED_NATION } else { p.addr() + staff::DATE_JOINED_CLUB };
    exe::date_operator_3d(joined, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    club.set_manager(p);
    if !national {
        p.set_club_job(club);
        let mut job = 5;
        let pl = p.player();
        if !pl.is_null() && (club.reputation() as i32) < pl.current_reputation() as i32 + 0x2ee {
            if let Some(i) = (0..50).find(|&i| club.squad(i) == 0) {
                covered!("sma_2 player-manager");
                club.set_squad(i, p.addr());
                job = 0xc; // sma2: the player-manager
            }
        }
        p.set_job_for_club(job);
    } else {
        p.set_national_job(club);
        p.set_job_for_nation(5);
    }
    if human {
        exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), p);
        exe::training_manager_reassign_squad_schedules(read_usize(G_TRAINING_MANAGER), p, old, club.addr() as u32); // sma2: the human's training
    }
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(appoint_fns::RESET_BOARD_FOR_NEW_MANAGER);
    reset(this, club.addr()); // sma2: the new board
    if !national {
        for i in 0..50 {
            let s = Staff(club.squad(i));
            if s.is_null() || s.player().is_null() {
                continue;
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
            if c == 0 {
                continue;
            }
            for bit in NEW_MANAGER_CLEARED_GRIEVANCES {
                if read_u32(c + 0x45) & bit != 0 {
                    exe::staff_contract_clear_grievances(c, bit); // sma2: the grievances the new manager clears
                }
            }
        }
    }
}

/// Comparer for `appoint_manager_to_club`.
///
/// The move cannot run twice (news, vacancies, contracts): only the original runs, and the outcome it guarantees is
/// checked: the club's `.manager()` is the person, with his job at the club or national team set (5, or 12 for a
/// player-manager in the club's squad).
pub(crate) unsafe extern "thiscall" fn compare_appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, club, manager);
    let c = Club(club);
    let p = Staff(manager);
    let ok = if exe::database_is_national_team(c) != 0 {
        p.national_job() == c && p.job_for_nation() == 5
    } else {
        p.club_job() == c && (p.job_for_club() == 5 || (p.job_for_club() == 12 && (0..50).any(|i| c.squad(i) == manager)))
    };
    SCOREBOARD.record(c.manager() == p && ok, || format!("appoint_manager_to_club(club {:x}, manager {:x}): manager {:x}, club job {:x}/{}, national job {:x}/{}",
        club, manager, c.manager().addr(), p.club_job().addr(), p.job_for_club(), p.national_job().addr(), p.job_for_nation()));
}
