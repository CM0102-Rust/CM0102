// ---------- batch %%LEVA%%: the board's daily pass and the manager appointments (manager_manager.cpp) ----------

/// `MANAGER_MANAGER::post_job_offer_to_human_news` (0x6a4f80; thiscall(this, club, person), ret 8): the job offer to a
/// human manager.
const FN_POST_JOB_OFFER_TO_HUMAN_NEWS: usize = 0x6a4f80;

/// The vacancy item (`manager_manager_fields::VACANCY_LIST`, 0x26 bytes): the rumour flags (+0 posted, +1 done), the
/// linked candidate (+2), the chosen person (+6), the club (`job_vacancy::CLUB_ID`), the arrival date (+0x16 day,
/// +0x18 year) and the rumour date (+0x1e day, +0x20 year).
#[derive(Clone, Copy)]
struct VacancyItem(usize);

impl VacancyItem {
    unsafe fn person(self) -> Staff {
        staff_at(read_i32(self.0 + 6) as isize)
    }
    unsafe fn club(self) -> Club {
        club_at(read_i32(self.0 + job_vacancy::CLUB_ID) as isize)
    }
    /// True once today is past the date at `day` (+0 day, +2 year).
    unsafe fn date_passed(self, day: usize) -> bool {
        let (today, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
        let year_due = read_u16(self.0 + day + 2);
        year > year_due || (year == year_due && today > read_i16(self.0 + day))
    }
}

/// Sets the club's `board_record::JOB_APPLICATION_DATE` to "never" (23/07/1971).
unsafe fn clear_job_application_date(this: usize, club: Club) {
    let mut never = [0u16; 4];
    let d = exe::date_date_549f70(&mut never, 0x17, 7, 0x7b3, u32::MAX);
    manager_manager_copy_date(board_row(this, club) + board_record::JOB_APPLICATION_DATE, d);
}

/// True for the 16 human managers (the last ids of the staff table).
unsafe fn is_human_id(person: Staff) -> bool {
    read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) - 0x10
}

/// Frees the person's staff job at `from` when he has a manager record (`remove_staff_from_club`).
unsafe fn remove_staff(this: usize, person: Staff, from: Club) {
    if !person.non_player().is_null() {
        remove_staff_from_club(this, person.addr(), from.addr());
    }
}

/// Takes the person out of the squad of `from` when he has a player record (`manager_manager_remove_player_from_squad`).
unsafe fn remove_player(this: usize, person: Staff, from: Club) {
    if !person.player().is_null() {
        manager_manager_remove_player_from_squad(this, person.addr(), from.addr());
    }
}

/// The 16 human managers who joined their club more than 180 days ago lose the club board's
/// `board_record_flags::NEW_MANAGER` bit (bit 0 of `board_record::FLAGS`).
unsafe fn settle_new_human_managers(this: usize) {
    let mut since = [0u16; 4];
    exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 0xb4);
    for k in 0..0x10 {
        let person = staff_at((read_i32(G_STAFF_COUNT) - 0x10 + k) as isize);
        let club = person.club_job();
        if club.is_null() || person.classification() == 0 {
            continue;
        }
        let year = person.date_joined_club_year();
        if since[1] > year || (since[1] == year && (since[0] as i16) > read_i16(person.addr() + staff::DATE_JOINED_CLUB)) {
            let flags = board_row(this, club) + board_record::FLAGS;
            write_u8(flags, read_u8(flags) & 0xfe);
        }
    }
}

/// A human manager's vacancy whose arrival date came: without a classification the vacancy reopens; without a club or
/// out of work for 90 days the club offers him the job (`FN_POST_JOB_OFFER_TO_HUMAN_NEWS`) and the vacancy reopens;
/// otherwise the vacancy reopens too (the order of the list calls is the original's).
unsafe fn human_vacancy_reached(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if person.classification() == 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else if person.club_job().is_null()
        || exe::date_days_since(G_CURRENT_DATE, person.date_joined_club() as u32, person.date_joined_club_leap_year() as u32) >= 0x5a
    {
        covered!("sx14_3 offer to the human");
        let offer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_POST_JOB_OFFER_TO_HUMAN_NEWS);
        offer(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else {
        clear_job_application_date(this, club);
        add_job_vacancy(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
    }
}

/// The club the appointed person leaves: his national team (for a national team, unless his job there is 11), else
/// his club; an AI manager going from a club to a national team with reputation above 6000 that is not a B team
/// (`exe::club_is_national_team`) leaves his club too. None otherwise.
unsafe fn previous_club(person: Staff, club: Club) -> Option<Club> {
    let national = exe::database_is_national_team(club) != 0;
    if national && !person.national_job().is_null() && person.job_for_nation() != 0xb {
        return Some(person.national_job());
    }
    if !national && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    if !is_human_id(person) && national && exe::club_is_national_team(club) == 0 && club.reputation() > 0x1770 && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    None
}

/// Moving to a club from his own club: frees his jobs there by his club job (`staff::JOB_FOR_CLUB`) and returns the
/// appointment kind (1 a player, 3 a manager or player-manager, 2 otherwise).
unsafe fn leave_club_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_club() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            3
        }
        0xf => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if person.player().is_null() {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            } else {
                remove_player(this, person, from);
            }
            2
        }
    }
}

/// An AI person moving to a club from his national team: frees his jobs by his national job
/// (`staff::JOB_FOR_NATION`), although the default case writes his club job, and returns the appointment kind.
unsafe fn leave_national_team_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 | 0xc => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 | 0xf => {
            remove_staff(this, person, from);
            2
        }
        0xb => 1,
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Moving to a national team: frees his jobs at the club or national team he leaves by his national job and returns
/// the appointment kind; a job 15 without a player record shows the original's error box (line 0x6bb).
unsafe fn leave_for_national_team(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        0xf => {
            if person.player().is_null() {
                show_error_at(0x9f18a4, 0x6bb);
            } else {
                remove_player(this, person, from);
            }
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_nation(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Posts the appointment and frees the person from the club he leaves (`leave_*`), ending his contract
/// (`exe::contract_manager_remove_contract`).
unsafe fn release_appointed_person(this: usize, person: Staff, previous: Option<Club>, club: Club) {
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let appoint = |from: usize, kind: u32| post_manager_appointment_news(this, person.addr(), from, club.addr(), kind);
    if exe::database_is_national_team(club) == 0 {
        match previous {
            Some(from) if from == person.club_job() => {
                let kind = leave_club_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            Some(from) if !is_human_id(person) && from == person.national_job() => {
                let kind = leave_national_team_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            _ => appoint(0, 2),
        }
    } else {
        match previous {
            Some(from) if from == person.national_job() || from == person.club_job() => {
                covered!("sx14_3 to a national team");
                let kind = leave_for_national_team(this, person, from);
                appoint(from.addr(), kind);
                if exe::database_is_national_team(from) == 0 {
                    exe::contract_manager_remove_contract(contracts, person);
                }
            }
            _ => appoint(0, 2),
        }
    }
}

/// A player moving between two clubs goes into the past transfers (`exe::transfer_manager_add_past_transfer`): the
/// person, his three name ids (−1 without a name), the new and the old club, no fee, type 5 and today.
unsafe fn record_manager_move(person: Staff, previous: Club, club: Club) {
    let mut rec = [0u8; 0x25];
    let r = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(r + 0x1d);
    write_i32(r, read_i32(person.addr()));
    for (k, name) in [person.first_name(), person.second_name(), person.common_name()].into_iter().enumerate() {
        write_i32(r + 4 + 4 * k, if name == 0 { -1 } else { read_i32(name + names::ID) });
    }
    write_i32(r + 0x10, read_i32(club.addr()));
    write_i32(r + 0x14, read_i32(previous.addr()));
    write_i32(r + 0x18, 0); // the fee
    write_u8(r + 0x1c, 5); // a non-player job
    manager_manager_copy_date(r + 0x1d, G_CURRENT_DATE);
    exe::transfer_manager_add_past_transfer(read_usize(G_TRANSFER_MANAGER), rec.as_mut_ptr());
}

/// The person takes the club: the joining date, the club's manager, and at a club the job (a player-manager, job 0xc,
/// with a free squad place, his reputation + 750 above the club's, fewer than 45 in the squad and outside Italy; else
/// manager, job 5) and the board's wealth band; at a national team the national job (5).
unsafe fn install_manager(this: usize, person: Staff, club: Club) {
    if exe::database_is_national_team(club) != 0 {
        manager_manager_copy_date(person.date_joined_nation_addr(), G_CURRENT_DATE);
    } else {
        manager_manager_copy_date(person.addr() + staff::DATE_JOINED_CLUB, G_CURRENT_DATE);
    }
    club.set_manager(person);
    let row = board_row(this, club);
    if exe::database_is_national_team(club) != 0 {
        person.set_national_job(club);
        write_u8(row + board_record::WEALTH_BAND, 0);
        person.set_job_for_nation(5);
        return;
    }
    person.set_club_job(club);
    let player = person.player();
    let nation = club.nation();
    let free_place = if !player.is_null()
        && (club.reputation() as i32) < player.current_reputation() as i32 + 0x2ee
        && (exe::database_count_club_squad(club, 0, 0) as i8) < 0x2d
        && !nation.is_null()
        && read_i32(nation.addr()) != read_i32(KEY_NATION_ITALY)
    {
        (0..0x32).find(|&k| club.squad(k) == 0)
    } else {
        None
    };
    match free_place {
        Some(k) => {
            covered!("sx14_3 player-manager");
            club.set_squad(k, person.addr());
            person.set_job_for_club(0xc);
            if exe::database_is_national_team(club) == 0 {
                let contracts = read_usize(G_CONTRACT_MANAGER);
                exe::contract_manager_remove_contract(contracts, person);
                exe::contract_manager_add_staff_contract(contracts, person, person.value() as u32);
                let squads = read_usize(G_SQUAD_MANAGER);
                exe::squad_manager_set_squad_number(squads, person, club, 0);
                if exe::squad_manager_club_uses_squad_numbers(squads, club, 0) != 0 {
                    exe::squad_manager_allocate_squad_number(squads, club, 0usize);
                }
            }
        }
        None => person.set_job_for_club(5),
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    write_u8(row + board_record::WEALTH_BAND, if finance != 0 { exe::finance_get_wealth_band(finance) } else { 0 });
}

/// An AI person's vacancy whose arrival date came: without a manager record the vacancy reopens with him; on loan or
/// retiring (staff state 0x30) without both records it reopens without him; a club that already has a manager, or
/// a person under offer, just drops the item. Otherwise the appointment: the club he leaves
/// (`previous_club`; if he managed it and it can be managed, its season expectations and a vacancy there), the job
/// application date cleared, the release and the appointment news (`release_appointed_person`), the past transfer of a
/// player between two clubs (`record_manager_move`), the installation (`install_manager`), the item off the list and
/// the board reset (`reset_board_for_new_manager`).
unsafe fn appoint_from_vacancy(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if is_human_id(person) {
        human_vacancy_reached(this, list, item, person, club);
        return;
    }
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let reopen = |left: usize| {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), left);
    };
    if person.non_player().is_null() {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
        return;
    }
    if exe::contract_manager_get_loan_contract(contracts, person) != 0
        || (staff_state_byte_b(read_i32(person.addr())) & 0x30 != 0 && (person.non_player().is_null() || person.player().is_null()))
    {
        reopen(0);
        return;
    }
    if exe::database_get_club_manager(club) != 0 || exe::transfer_manager_is_player_under_offer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        return;
    }
    covered!("sx14_3 appointment");
    let previous = previous_club(person, club);
    if let Some(from) = previous {
        if from.manager() == person && exe::club_can_be_managed(from) != 0 {
            set_season_expectations(this, from.addr(), 1);
            add_job_vacancy(this, from.addr(), person.addr());
        }
    }
    clear_job_application_date(this, club);
    if person.non_player().is_null() {
        show_error_at(0x9f18a4, 0x5e2);
    }
    release_appointed_person(this, person, previous, club);
    if let Some(from) = previous {
        if !person.player().is_null() && exe::database_is_national_team(from) == 0 && !club.is_null() && exe::database_is_national_team(club) == 0 {
            record_manager_move(person, from, club);
        }
    }
    install_manager(this, person, club);
    // simplification: the original then syncs a human's scout lists and training schedules (0x7f7270, 0x8b2470), a
    // test that never holds here (the humans took `human_vacancy_reached`)
    exe::llist_remove(list, item.0);
    reset_board_for_new_manager(this, club.addr());
}

/// A vacancy still before its arrival date: once its rumour date passed, the rumour linking the candidate with the
/// club is posted once (`post_job_rumour_news`), while the club has no manager.
unsafe fn vacancy_rumour(this: usize, item: VacancyItem) {
    let e = item.0;
    if read_u8(e) == 0 || !item.date_passed(0x1e) || read_u8(e + 1) != 0 || read_i32(e + 2) == -1 {
        return;
    }
    write_u8(e + 1, 1);
    let club = item.club();
    if club.manager().is_null() {
        covered!("sx14_3 rumour");
        post_job_rumour_news(this, staff_at(read_i32(e + 2) as isize).addr(), club.addr());
    }
}

/// The board's daily pass over the clubs and the manager market.
///
/// `MANAGER_MANAGER::daily_update()`, every day (the game's day loop). When the month changes (`exe::date_get_month`
/// against the last month seen at 0xb67a40; the first day after loading only records it, flag 0x9f18a0), each club that
/// is not a national team, or a national team whose nation has +0x71 set, gets the board's confidence news
/// (`post_board_confidence_update_news`), then `fix_missing_jobs`. The human managers settle in after 180 days
/// (`settle_new_human_managers`); on Sundays (day of the year divisible by 7) `weekly_board_confidence_update`; the
/// pending departures (`process_pending_departures`). Then every vacancy of `manager_manager_fields::VACANCY_LIST`
/// from the head: past its arrival date, the appointment (`appoint_from_vacancy`); before it, the rumour
/// (`vacancy_rumour`). Finally the ban appeals (`process_ban_appeals`).
///
/// Defects of the original: a person moving to a club from his national team is released by his national job, but
/// the default case writes his club job (0xb, 0 or 8); the "manager record" check after the appointment can only show
/// its error box (line 0x5e2) if a call in between cleared the record.
/// Original: `MANAGER_MANAGER::daily_update` (0x68f9c0, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn daily_update(this: usize) {
    const FIRST_DAY_AFTER_LOAD: usize = 0x9f18a0;
    const LAST_MONTH: usize = 0xb67a40;
    let month = exe::date_get_month(G_CURRENT_DATE);
    if read_u8(FIRST_DAY_AFTER_LOAD) != 0 {
        write_u8(LAST_MONTH, month);
        write_u8(FIRST_DAY_AFTER_LOAD, 0);
    } else if month != read_u8(LAST_MONTH) {
        covered!("sx14_3 new month");
        write_u8(LAST_MONTH, exe::date_get_month(G_CURRENT_DATE));
        let mut k = 0;
        while k < read_i32(G_CLUB_COUNT) {
            let club = club_at(k as isize);
            let nation = club.nation();
            if exe::database_is_national_team(club) == 0 || (!nation.is_null() && read_u32(nation.addr() + 0x71) != 0) {
                post_board_confidence_update_news(this, club.addr());
            }
            k += 1;
        }
        fix_missing_jobs(this);
    }
    settle_new_human_managers(this);
    if read_i16(G_CURRENT_DATE) % 7 == 0 {
        weekly_board_confidence_update(this);
    }
    process_pending_departures(this);
    let list = read_usize(this + manager_manager_fields::VACANCY_LIST);
    list_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        let item = VacancyItem(e);
        if item.date_passed(0x16) {
            appoint_from_vacancy(this, list, item, item.person(), item.club());
        } else {
            vacancy_rumour(this, item);
        }
    }
    process_ban_appeals(this);
}

/// Comparer for `daily_update`: it moves managers, posts news and draws, so only the original runs (its effects are
/// checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_daily_update(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
