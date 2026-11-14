// ---------- batch 294: the retirement announcement text (player_regen.cpp) ----------

/// The job name of a role and its gender (`database_get_job_name`, called through the exe at 0x53ad00: long, lower
/// case).
pub(crate) unsafe fn retirement_job_name(role: u8, gender: &mut i8) -> usize {
    let f: unsafe extern "C" fn(u32, u32, u32, *mut i8) -> usize = core::mem::transmute(0x53ad00usize);
    f(role as u32, 0, 0, gender)
}

/// Builds the title and body text of the news that a staff member announces his decision to retire.
///
/// `player_regen_write_retirement_announcement_text(news, body, out)`: slots 0 (the person's id), 1..3 (the name ids),
/// 4 (the kind: 1, 2, 0x10, 0x20), 5 (the club id, −1 none) and 6 (an age). The name (`exe::database_format_name_from_ids`,
/// 0x190, style 5 in the headline, 1 in the body). Headline: 0xa70110 with the name. Body: when the club's manager
/// (`club::MANAGER`) is the human of the current screen slot (`G_SCREEN_RECORDS_MANAGER` + 0x300 ×
/// `G_SCREEN_MANAGER_CURRENT_SLOT`), the texts addressed to him (only the name): 0x10 without a non-player sheet by the
/// age (`staff::AGE` below 27 0xa6fb74, else 0xa6fac8), with one by the club job (0/0x10 0xa6fc1c, 0xc 0xa6fd20, 0xd
/// 0xa6fe1c, 0xf 0xa6ff24, other 0xa70020); 1 without a sheet 0xa6f57c, with one by the club job (0/0x10/0xb 0xa6e5cc,
/// 0xc 0xa6f628, 0xd 0xa6f730, 0xf 0xa6f840, other by slot 6 below 28 0xa6f944, else 0xa6fa04); 0x20 with a club job
/// 0xa6f4b0 (the job's name, `database_get_job_name`), with a national job 0xa6f3cc (the national team's gender and
/// name), else 0xa6f338; 2 0xa6e24c. Otherwise the club texts (the club's short gender and short name, the original
/// reads them even without a club for kind 0x10): 0x10 as above (0xa6ec2c / 0xa6eb44 by the age; 0xa6ed10, 0xa6ee4c,
/// 0xa6ef84, 0xa6f0c8, 0xa6f200 by the job); 1 without a sheet 0xa6e4fc with a club or 0xa6f57c, with one 0xa6e6cc,
/// 0xa6e7f8, 0xa6e930, 0xa6ea5c by the job at a club, else 0xa6e5cc; 0x20 with a club and a club job 0xa6e408, with a
/// national job 0xa6f3cc, else 0xa6e35c; 2 with a club 0xa6e2c0, else 0xa6e24c. Other kinds leave the buffer
/// 0xde1f64 as it was. Then the text (`exe::text_set_string`) and the person's link (0x7d7) when the ids still match
/// (`exe::database_staff_matches_names`).
/// Original: `player_regen_write_retirement_announcement_text` (0x7aa4c0, player_regen.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn player_regen_write_retirement_announcement_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let buf = 0xde1f64usize;
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let ids = [exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2), exe::newsitem_get_slot(news, 3)];
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, 5, id);
        format(buf, 0xa70110, nm);
    } else {
        covered!("sma_8 retirement body");
        let kind = exe::newsitem_get_slot(news, 4) as u8;
        let club_id = exe::newsitem_get_slot(news, 5) as i32;
        let age = exe::newsitem_get_slot(news, 6) as i32;
        let club = if club_id != -1 { read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE)) } else { 0 };
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, 1, id);
        let human = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
        let at_human = club != 0 && read_usize(club + club::MANAGER) != 0 && read_usize(club + club::MANAGER) == human;
        let sheet = !person.non_player().is_null();
        let role = person.job_for_club();
        let say = |t: usize| {
            format(buf, t, nm);
        };
        let g = || read_i8(club + club::SHORT_GENDER_NAME) as i32;
        let say_club = |t: usize| {
            format(buf, t, g(), g(), club + club::SHORT_NAME, nm);
        };
        // the national job text of kind 0x20, when the person has one; false otherwise
        let national = || {
            let nr = person.job_for_nation();
            let nj = person.national_job();
            if nr != 0 && nr != 0x10 && !nj.is_null() {
                let mut gt: i8 = 0;
                let title = retirement_job_name(nr, &mut gt);
                let gn = read_i8(nj.addr() + club::GENDER_NAME) as i32;
                format(buf, 0xa6f3cc, gn, gn, gt as i32, gt as i32, nm, nj.addr() + club::NAME, title);
                true
            } else {
                false
            }
        };
        let by_job = |t: [usize; 5]| match role {
            0 | 0x10 => t[0],
            0xc => t[1],
            0xd => t[2],
            0xf => t[3],
            _ => t[4],
        };
        if at_human {
            match kind {
                0x10 => say(if !sheet {
                    if (person.age() as i8) < 0x1b { 0xa6fb74 } else { 0xa6fac8 }
                } else {
                    by_job([0xa6fc1c, 0xa6fd20, 0xa6fe1c, 0xa6ff24, 0xa70020])
                }),
                1 => say(if !sheet {
                    0xa6f57c
                } else if role == 0 || role == 0x10 || role == 0xb {
                    0xa6e5cc
                } else {
                    by_job([0, 0xa6f628, 0xa6f730, 0xa6f840, if age < 0x1c { 0xa6f944 } else { 0xa6fa04 }])
                }),
                0x20 => {
                    if role != 0 && role != 0x10 && !person.club_job().is_null() {
                        let mut gt: i8 = 0;
                        let title = retirement_job_name(role, &mut gt);
                        format(buf, 0xa6f4b0, gt as i32, gt as i32, nm, title);
                    } else if !national() {
                        say(0xa6f338);
                    }
                }
                2 => say(0xa6e24c),
                _ => {}
            }
        } else {
            match kind {
                0x10 => say_club(if !sheet {
                    if (person.age() as i8) < 0x1b { 0xa6ec2c } else { 0xa6eb44 } // sma8: the young retiring player
                } else {
                    by_job([0xa6ed10, 0xa6ee4c, 0xa6ef84, 0xa6f0c8, 0xa6f200])
                }),
                1 => {
                    if !sheet {
                        if club != 0 { say_club(0xa6e4fc) } else { say(0xa6f57c) }
                    } else if role != 0 && role != 0x10 && role != 0xb && club != 0 {
                        say_club(by_job([0, 0xa6e6cc, 0xa6e7f8, 0xa6e930, 0xa6ea5c]));
                    } else {
                        say(0xa6e5cc);
                    }
                }
                0x20 => {
                    if club != 0 && role != 0 && role != 0x10 && !person.club_job().is_null() {
                        let mut gt: i8 = 0;
                        let title = retirement_job_name(role, &mut gt);
                        format(buf, 0xa6e408, g(), g(), gt as i32, gt as i32, club + club::SHORT_NAME, nm, title);
                    } else if !national() {
                        say(0xa6e35c);
                    }
                }
                2 => {
                    if club != 0 { say_club(0xa6e2c0) } else { say(0xa6e24c) }
                }
                _ => {}
            }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
}

/// Comparer for `player_regen_write_retirement_announcement_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_write_retirement_announcement_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_PLAYER_REGEN_WRITE_RETIREMENT_ANNOUNCEMENT_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| player_regen_write_retirement_announcement_text(news, body, t),
        || format!("player_regen_write_retirement_announcement_text(body {})", body));
}

// ---------- batch 294: the season update pass (game.cpp) ----------

/// Fields of the 72-byte season entries (`G_LEAGUE_TABLE`, `key_nation_record`) that `fields.rs` does not name yet, read
/// from `game_season_update_pass` (0x5d5ae0). [inference] for the names: the four `Date::Date(day, month, year, weekday)`
/// arguments rebuild `START_DATE` (+0x2a) and `END_DATE` (+0x39).
#[allow(dead_code)]
pub(crate) mod season_entry {
    pub(crate) const COMP_COUNT: usize = 0xc; // i32: number of competitions of the entry
    pub(crate) const COMPS: usize = 0x10; // pointer to the array of the entry's competition objects
    pub(crate) const START_DAY: usize = 0x32; // u16: day of the month of the start date
    pub(crate) const START_YEAR: usize = 0x35; // u16: year of the start date (compared with today's year)
    pub(crate) const START_WEEKDAY: usize = 0x37; // u16: weekday the start date snaps to (0xffff: none)
    pub(crate) const END_DAY: usize = 0x41; // u16: day of the month of the end date
    pub(crate) const END_MONTH: usize = 0x43; // u8: month of the end date
    pub(crate) const END_YEAR: usize = 0x44; // u16: year of the end date
    pub(crate) const END_WEEKDAY: usize = 0x46; // u16: weekday the end date snaps to (0xffff: none)
}

/// The exe routines and globals the season update pass uses that `exe_fns.rs` / `globals.rs` do not name yet.
#[allow(dead_code)]
pub(crate) mod season_pass {
    /// `g_season_update_pass_last_date`: the date of the last SimulatedStats rebuild (static local Date).
    pub(crate) const LAST_DATE: usize = 0xae2c80;
    pub(crate) const LAST_DATE_YEAR: usize = 0xae2c82;
    /// `g_season_update_pass_init_flag`: bit 0 = `LAST_DATE` was built.
    pub(crate) const INIT_FLAG: usize = 0xae2c89;
    /// `season_update_static_date_dtor` (empty at-exit destructor of `LAST_DATE`).
    pub(crate) const FN_LAST_DATE_DTOR: usize = 0x5d5f10;
    /// "Updating game data" (the progress text, through langlib_sprintf).
    pub(crate) const TEXT_UPDATING_GAME_DATA: usize = 0x9a820c;
    /// `Club_Records::end_season_for_nation` (thiscall(this, entry), ret 4).
    pub(crate) const FN_CLUB_RECORDS_END_SEASON: usize = 0x447880;
    /// `PlayerStats` end of season for a key nation (thiscall(this, entry), ret 4; Rust `end_season_for_key_nation`).
    pub(crate) const FN_PLAYER_STATS_END_SEASON: usize = 0x7c1350;
    /// `FinanceTable::end_season_for_nation` (thiscall(this, entry), ret 4; Rust `fin_prm_end_season_for_nation`).
    pub(crate) const FN_FINANCE_END_SEASON: usize = 0x59cdd0;
    /// `Staff_Records::end_season_for_nation` (thiscall(this, entry), ret 4; Rust `staff_records_end_season_for_nation`).
    pub(crate) const FN_STAFF_RECORDS_END_SEASON: usize = 0x86d6e0;
    /// The average attendance of a league division (cdecl(comp), Rust `cqt4_update_average_attendance`).
    pub(crate) const FN_UPDATE_AVERAGE_ATTENDANCE: usize = 0x5d6280;
    /// `Discipline::start_new_season_for_comp` (thiscall(this, comp record), ret 4; Rust `discipline_new_season`).
    pub(crate) const FN_DISCIPLINE_NEW_SEASON: usize = 0x556210;
    /// `SimulatedStats::~SimulatedStats` (thiscall(this): frees +4).
    pub(crate) const FN_SIMULATED_STATS_DTOR: usize = 0x84be10;
    /// `SimulatedStats::SimulatedStats` (thiscall(this, 0), ret 4; returns this).
    pub(crate) const FN_SIMULATED_STATS_CTOR: usize = 0x84b940;
    /// Comp vtable slot 0xc: `Comp::all_stages_complete` (bool in al).
    pub(crate) const VT_ALL_STAGES_COMPLETE: usize = 0xc;
    /// Comp vtable slot 0x2c: `skip_season_reset` (0 by default; EuroChampQual and WcConcacafCup return 1 after marking
    /// themselves).
    pub(crate) const VT_SKIP_SEASON_RESET: usize = 0x2c;
}

/// Sends the "Updating game data" progress step (`ScreenManager::send_network_status_message(text, step, total, -1)`),
/// only when the pass is heavy (weight 10 or more) and the front end is interactive.
unsafe fn season_pass_progress(text: &[u8; 0xb8], step: i32, weight: i32) {
    if weight >= 10 && read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, step, weight as u32, -1);
    }
}

/// The day before an entry's update day: the nation's records are archived (club records, player stats, finances,
/// staff records), one progress step after each.
unsafe fn season_pass_day_before(e: usize, text: &[u8; 0xb8], step: &mut i32, weight: i32) {
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
unsafe fn season_pass_move_dates(e: usize, nation: usize) {
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
unsafe fn season_pass_restart_comp(comp: usize) {
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

// ---------- batch 294: the media story about a player (media.cpp) ----------

std::thread_local! {
    /// The pieces of `write_player_media_story_news_text` its last call ran, in order (the comparer names them).
    static PLAYER_STORY_TRAIL: core::cell::RefCell<Vec<&'static str>> = core::cell::RefCell::new(Vec::new());
}

/// Notes the piece that is running.
fn player_story_piece(name: &'static str) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().push(name));
}

/// The exe functions the player media story calls that `exe_fns.rs` does not name yet (Rust ports, called through the
/// game's address so their own detours and comparers stay in the path).
const PLAYER_STORY_POSITION_MASK_GET_TEXT: usize = 0x5482f0; // position_mask_get_text, cdecl
const PLAYER_STORY_WRITE_POSITION_NAME: usize = 0x5461b0; // write_position_name, cdecl
const PLAYER_STORY_WRITE_PLAYER_DESCRIPTION: usize = 0x5465a0; // write_player_description, cdecl
const PLAYER_STORY_GET_PUNDIT_NAME: usize = 0x7dad20; // PunditList::get_pundit_name, thiscall, ret 0x10
const PLAYER_STORY_FIXTURE_ENTRY_MATCHES: usize = 0x729500; // Match_Stats::fixture_entry_matches, thiscall, ret 0x14

/// The game's text formatter (`FN_LANGLIB_SPRINTF`, cdecl, variadic: every argument a dword).
macro_rules! story_lang {
    ($buf:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        f(($buf) as usize, ($tpl) as usize $(, ($a) as usize)*);
    }};
}

/// The C runtime's sprintf (`FN_CRT_SPRINTF`).
macro_rules! story_crt {
    ($buf:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(($buf) as usize, ($tpl) as usize $(, ($a) as usize)*);
    }};
}

/// `record_utils` 0x7e4fa0 (cdecl): reads a fixture record (0x1f bytes: date, competition, teams, score, clubs) from the
/// news item's slots, starting at `*counter` and moving it on.
const PLAYER_STORY_READ_FIXTURE: usize = 0x7e4fa0;

/// strcat: appends `src` to the string at `dst`.
unsafe fn story_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// The locals of the original's body path (the slots of the news item, the names written once and the buffers).
struct PlayerStory {
    news: usize,
    /// slot 0: the player (staff index)
    player: i32,
    /// slots 1..3: the player's name ids
    names: [i32; 3],
    /// slot 4: the player's club (club index; -1 none)
    club: i32,
    /// slot 5: the variant of the story
    subtype: i32,
    /// slot 6: the outlet type (1..6, other: website)
    outlet: i32,
    /// slot 7 (unused by the text)
    slot7: i32,
    /// slot 8 (unused by the text)
    slot8: i32,
    /// slot 9: the story kind
    kind: i32,
    /// slot 10: the reader paragraph's case
    reader_case: i32,
    /// slot 11: the second person's index (-1 none)
    second: i32,
    /// slots 12..14: the second person's name ids
    second_names: [i32; 3],
    /// slot 14 read again (the original asks for 14 twice and never for 15)
    slot14_again: i32,
    slot16: i32,
    slot17: i32,
    slot18: i32,
    /// slot 23: the third person (staff index; the human manager when it is him)
    third: i32,
    /// slots 24..26: the third person's name ids
    third_names: [i32; 3],
    slot27: i32,
    /// slot 28: the story comes with a match (fixture record from slot 29)
    with_fixture: i32,
    /// the fixture record (date, ..., +0x15 the other club, +0x19 and +0x1a the goals)
    fixture: [u8; 0x20],
    /// with a fixture, the slot after it (the result code 1..4); without one, never written
    result: i32,
    /// the player's qualities (for `write_player_description`)
    qualities: i32,
    /// the player's position mask (for `write_position_name` and the description)
    position: i32,
    /// the pundit's id (word; -1 none)
    pundit: i32,
    /// the position mask for `position_mask_get_text`
    position_mask: i32,
    byte_3272: u8,
    byte_3271: u8,
    /// the fourth person (-1 none) and his name ids
    fourth: i32,
    fourth_names: [i32; 3],
    /// the player's offset in the staff table and the third person's
    player_off: usize,
    third_off: usize,
    /// the player's club is in the nation of the human manager on screen (the "local" media)
    local: u32,
    /// the outlet's grammar mark
    mark: u8,
    /// the gender byte of the position name (only the low byte is set; the original passes the dword on)
    position_gender: u8,
    /// the gender byte of the position mask text
    mask_gender: u8,
    name: [u8; 0xc8],
    name5: [u8; 0x78],
    third_name: [u8; 0xf0],
    third_name5: [u8; 0x78],
    second_name: [u8; 0xc8],
    fourth_name: [u8; 0x78],
    position_text: [u8; 0xc8],
    mask_text: [u8; 0x200],
    outlet_name: [u8; 0x200],
    adjective: [u8; 0x80],
    score: [u8; 0x38],
    text: [u8; 0x1388],
    extra: [u8; 0x400],
    long_extra: [u8; 0x1588],
}

impl PlayerStory {
    /// The club record of the story (`G_CLUB_TABLE` + `club::SIZE` × index).
    unsafe fn club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.club as usize).wrapping_mul(club::SIZE))
    }

    /// The club's short name and its gender byte (sign extended).
    unsafe fn club_short(&self) -> (usize, i32) {
        let c = self.club_rec();
        (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32)
    }

    /// The club's nation record (`club::NATION`, a pointer after the load).
    unsafe fn club_nation(&self) -> usize {
        read_usize(self.club_rec() + club::NATION)
    }

    /// The other club of the fixture (+0x15).
    unsafe fn other_club(&self) -> usize {
        let i = u32::from_le_bytes([self.fixture[0x15], self.fixture[0x16], self.fixture[0x17], self.fixture[0x18]]);
        read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
    }

    /// The two goal counts of the fixture (+0x19, +0x1a), signed bytes.
    fn goals(&self) -> (i8, i8) {
        (self.fixture[0x19] as i8, self.fixture[0x1a] as i8)
    }

    /// The nationality adjective of the club's nation into `adjective` (`exe::nation_get_nationality_adjective`, size
    /// 0x78, the form given).
    unsafe fn club_adjective(&mut self, form: u32) {
        exe::nation_get_nationality_adjective(self.club_nation(), self.adjective.as_mut_ptr() as usize, 0x78, form);
    }

}


impl PlayerStory {
    /// Reads the slots, writes the names and the position texts, and works out whether the media is local.
    unsafe fn read(news: usize) -> PlayerStory {
        let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
        let mut st: PlayerStory = core::mem::zeroed();
        st.news = news;
        exe::date_date_54a110(st.fixture.as_mut_ptr() as usize);
        st.player = s(0);
        st.names = [s(1), s(2), s(3)];
        st.club = s(4);
        st.subtype = s(5);
        st.outlet = s(6);
        st.slot7 = s(7);
        st.slot8 = s(8);
        st.kind = s(9);
        st.reader_case = s(10);
        st.second = s(11);
        st.second_names = [s(12), s(13), s(14)];
        st.slot14_again = s(14); // the original asks for slot 14 twice (and never for 15)
        st.slot16 = s(16);
        st.slot17 = s(17);
        st.slot18 = s(18);
        st.third = s(23);
        st.third_names = [s(24), s(25), s(26)];
        st.slot27 = s(27);
        st.with_fixture = s(28);
        if st.with_fixture != 0 {
            let read_fixture: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(PLAYER_STORY_READ_FIXTURE);
            let mut counter: u8 = 0x1d;
            read_fixture(news, &mut counter, st.fixture.as_mut_ptr() as usize);
            let mut next = || {
                let k = counter;
                counter = counter.wrapping_add(1);
                s(k as i8)
            };
            st.result = next();
            st.qualities = next();
            st.position = next();
            st.pundit = next();
            st.position_mask = 0;
            st.byte_3272 = 0;
            st.byte_3271 = 0;
            st.fourth = -1;
            st.fourth_names = [-1; 3];
        } else {
            st.qualities = s(0x1d);
            st.position = s(0x1e);
            st.pundit = s(0x1f);
            st.position_mask = s(0x20);
            st.byte_3272 = s(0x21) as u8;
            st.byte_3271 = s(0x22) as u8;
            st.fourth = s(0x23);
            st.fourth_names = [s(0x24), s(0x25), s(0x26)];
        }
        let mask_text: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
        st.mask_gender = mask_text(st.position_mask as u32, st.mask_text.as_mut_ptr() as usize, 0);
        st.player_off = (st.player as usize).wrapping_mul(staff::SIZE);
        st.third_off = (st.third as usize).wrapping_mul(staff::SIZE);
        let staff_table = read_usize(G_STAFF_TABLE);
        let [a, b, c] = st.names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name.as_mut_ptr(), 0x78, 1, st.player as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name5.as_mut_ptr(), 0x78, 5, st.player as u32);
        let [a, b, c] = st.third_names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name.as_mut_ptr(), 0x78, 1, st.third as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name5.as_mut_ptr(), 0x78, 5, st.third as u32);
        // the original passes a fourth argument (1) the routine does not read
        let position_name: unsafe extern "C" fn(u32, usize, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_POSITION_NAME);
        st.position_gender = position_name(st.position as u32, st.position_text.as_mut_ptr() as usize, 0xc8, 1);
        if staff_table.wrapping_add(st.third_off) == staff_table.wrapping_add(st.player_off) {
            story_lang!(st.position_text.as_mut_ptr(), 0xa242e0); // "player-manager"
            st.position_gender = 0;
        }
        if st.second != -1 {
            // the player's index goes with the second person's names (the original's choice)
            let [a, b, c] = st.second_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.second_name.as_mut_ptr(), 0x78, 1, st.player as u32);
        } else {
            st.second_name[0] = 0;
        }
        if st.fourth == -1 {
            st.fourth_name[0] = 0;
        } else {
            let [a, b, c] = st.fourth_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.fourth_name.as_mut_ptr(), 0x78, 1, st.fourth as u32);
        }
        // the media is local when the player's club is in the nation of the human manager on screen: his club's nation,
        // or his own nation without a club
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        st.local = 0;
        if manager != 0 {
            let job = read_usize(manager + staff::CLUB_JOB);
            let nation = st.club_nation();
            if job != 0 {
                if read_usize(job + club::NATION) == nation {
                    st.local = 1;
                }
            } else if nation == read_usize(manager + staff::NATION) {
                st.local = 1;
            }
        }
        st
    }

    /// The outlet's name into `outlet_name` and its grammar mark: the pundit's name when the media is local and the
    /// story names one; otherwise the outlet type (local: "national newspaper"...; foreign: with the nation's adjective).
    unsafe fn write_outlet(&mut self) {
        player_story_piece("outlet");
        covered!("nma_3 outlet");
        let nation = self.club_nation();
        if self.club != -1 && self.outlet != -1 && self.pundit as i16 != -1 && nation != 0 && self.local != 0 {
            covered!("nma_3 pundit");
            let pundit_name: unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8) = core::mem::transmute(PLAYER_STORY_GET_PUNDIT_NAME);
            pundit_name(read_usize(G_PUNDIT_LIST), self.outlet as u32, self.pundit as u32, self.outlet_name.as_mut_ptr() as usize, &mut self.mark);
            return;
        }
        let out = self.outlet_name.as_mut_ptr();
        let (plain, form, foreign) = match self.outlet {
            1 => (0xa242cc, 0x16, 0xa24294), // national newspaper
            2 => (0xa24284, 0x17, 0xa24250), // local newspaper
            3 => (0xa24238, 0x18, 0xa24208), // national radio station
            4 => (0xa241f4, 0x18, 0xa241bc), // local radio station
            5 => (0xa241b0, 0x19, 0xa24184), // magazine
            6 => (0xa24174, 0x1a, 0xa24144), // sports paper
            _ => (0xa2413c, 0x1a, 0xa24110), // website
        };
        if self.local != 0 {
            story_lang!(out, plain);
        } else {
            self.club_adjective(form);
            story_lang!(out, foreign, 0x64, self.adjective.as_ptr());
        }
        self.mark = if self.outlet == 5 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
    }

    /// Kind 0x23: the player is fed up with being left out.
    unsafe fn left_out(&mut self) {
        player_story_piece("left out (0x23)");
        covered!("nma_3 left out");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 if self.local != 0 => story_lang!(text, 0xa23d38, g, g, 0x64, short, name),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa23c40, g, g, 0x64, 0x64, 0x64, short, name, self.adjective.as_ptr());
            }
            1 => story_lang!(text, 0xa23e0c, g, g, 0x64, short, name),
            2 => story_lang!(text, 0xa23efc, g, g, 0x64, short, name),
            _ => story_lang!(text, 0xa23ffc, mark, mark, g, g, name, self.outlet_name.as_ptr(), short),
        }
    }

    /// Kind 0xe: the media questions the player's ability.
    unsafe fn ability(&mut self) {
        player_story_piece("ability (0xe)");
        covered!("nma_3 ability");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let foreign = |st: &mut PlayerStory, form: u32, tpl: usize| {
            st.club_adjective(form);
            story_lang!(text, tpl, 0x64, 0x64, g, g, st.adjective.as_ptr(), name, short);
        };
        match self.subtype {
            0 if local => story_lang!(text, 0xa23b98, 0x64, g, g, name, short),
            0 => foreign(self, 0x16, 0xa23ad0),
            1 if local => story_lang!(text, 0xa23a20, 0x64, g, g, name, short),
            1 => foreign(self, 0x16, 0xa2395c),
            2 if local => story_lang!(text, 0xa2389c, 0x64, g, g, name, short),
            2 => foreign(self, 0x1e, 0xa237bc),
            3 => story_lang!(text, 0xa236ec, 0x64, g, g, name, short),
            _ => story_lang!(text, 0xa23628, g, g, 0x64, short, name),
        }
    }

    /// Kind 0xd: the player is accused of complacency.
    unsafe fn complacency(&mut self) {
        player_story_piece("complacency (0xd)");
        covered!("nma_3 complacency");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 => story_lang!(text, 0xa232c0, mark, mark, g, g, 0x64, outlet, short, name),
            1 => story_lang!(text, 0xa233b0, mark, mark, 0x64, g, g, outlet, name, short),
            2 => story_lang!(text, 0xa23474, mark, mark, 0x64, g, g, outlet, name, short),
            _ => story_lang!(text, 0xa23558, g, g, 0x64, short, name),
        }
    }

    /// The score of the fixture, the higher number first ("%d-%d").
    unsafe fn write_score(&mut self) {
        let (a, b) = self.goals();
        story_crt!(self.score.as_mut_ptr(), 0x98c114, a.max(b) as i32, a.min(b) as i32);
    }

    /// Kind 0x11: the media questions the player's contribution (after a match, with the score; without one, the
    /// supporters speak out).
    unsafe fn contribution(&mut self) {
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.with_fixture == 0 {
            player_story_piece("fan criticism (0x11 without a match)");
            covered!("nma_3 fan criticism");
            story_lang!(text, 0xa21cb4, g, g, 0x64, short, name);
            return;
        }
        player_story_piece("contribution (0x11)");
        covered!("nma_3 contribution");
        self.write_score();
        let description: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_PLAYER_DESCRIPTION);
        let dg = description(self.player, self.position as u32, self.qualities as u32, self.position_text.as_mut_ptr() as usize, 0xc8, self.position_gender as u32, 1) as i8 as i32;
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let c2name = c2 + club::NAME;
        let c2g = read_i8(c2 + club::GENDER_NAME) as i32;
        let (score, pos, outlet, name5) = (self.score.as_ptr(), self.position_text.as_ptr(), self.outlet_name.as_ptr(), self.name5.as_ptr());
        let nation = self.club_nation();
        let (nshort, ng) = (nation + nation::SHORT_NAME, read_i8(nation + nation::SHORT_GENDER_NAME) as i32);
        let local = self.local != 0;
        let by_score = |win: usize, draw: usize, loss: usize| if g1 > g2 { win } else if g1 == g2 { draw } else { loss };
        match self.subtype {
            0 => {
                let mark = self.mark as i8 as i32;
                self.slot27 = g;
                let tpl = by_score(0xa22094, 0xa21f7c, 0xa21e60);
                story_lang!(text, tpl, g, g, c2g, c2g, mark, mark, 0x64, short, score, c2name, outlet, name);
            }
            1 => {
                self.slot27 = g;
                if local {
                    let tpl = by_score(0xa22820, 0xa2258c, 0xa222fc);
                    story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, 0x64, short, score, c2name, pos, name);
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    let tpl = by_score(0xa226d4, 0xa2243c, 0xa221a8);
                    story_lang!(text, tpl, ng, ng, g, g, c2g, c2g, dg, dg, 0x64, nshort, short, score, c2name, pos, name);
                }
            }
            _ => {
                self.slot27 = g;
                if local {
                    if g1 > g2 {
                        story_lang!(text, 0xa23154, g, g, c2g, c2g, dg, dg, name, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22e34 } else { 0xa22b14 };
                        story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, name, short, score, c2name, pos);
                    }
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    if g1 > g2 {
                        story_lang!(text, 0xa22fa4, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22c84 } else { 0xa22964 };
                        story_lang!(text, tpl, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, score, c2name, pos);
                    }
                }
            }
        }
        self.draw_result();
    }

    /// After a drawn match (kinds 0x11 and 0xa): how the club eventually won or lost (the result code), appended.
    unsafe fn draw_result(&mut self) {
        let (g1, g2) = self.goals();
        if g1 != g2 {
            return;
        }
        let tpl = match self.result {
            1 => 0xa21e0c,
            2 => 0xa21684,
            3 => 0xa21dc0,
            4 => 0xa21d74,
            _ => return,
        };
        player_story_piece("draw result");
        covered!("nma_3 draw result");
        let (short, g) = self.club_short();
        story_lang!(self.extra.as_mut_ptr(), tpl, g, g, short);
        story_strcat(self.text.as_mut_ptr() as usize, self.extra.as_ptr() as usize);
    }

    /// Kind 0xc: the media questions the player's commitment.
    unsafe fn commitment(&mut self) {
        player_story_piece("commitment (0xc)");
        covered!("nma_3 commitment");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa21bdc, mark, mark, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x13: the player's indiscipline (with bit 1 of the position mask, without the position text).
    unsafe fn indiscipline(&mut self) {
        player_story_piece("indiscipline (0x13)");
        covered!("nma_3 indiscipline");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        if self.position_mask as u8 & 1 == 1 {
            story_lang!(text, 0xa21acc, mark, mark, g, g, outlet, short, name);
        } else {
            let mg = self.mask_gender as i8 as i32;
            story_lang!(text, 0xa21998, mark, mark, g, g, mg, outlet, short, name, self.mask_text.as_ptr());
        }
    }

    /// Kind 0xa with a match: the player's contribution in the match, with the score and the other club.
    unsafe fn contribution_after_match(&mut self) {
        player_story_piece("contribution after a match (0xa)");
        covered!("nma_3 contribution after a match");
        self.write_score();
        let (short, g) = self.club_short();
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let (c2name, c2g) = (c2 + club::NAME, read_i8(c2 + club::GENDER_NAME) as i32);
        let mark = self.mark as i8 as i32;
        let tpl = if g1 > g2 { 0xa21834 } else if g1 == g2 { 0xa216d0 } else { 0xa2151c };
        story_lang!(self.text.as_mut_ptr(), tpl, mark, mark, g, g, c2g, c2g, self.outlet_name.as_ptr(), short, self.name.as_ptr(),
            self.score.as_ptr(), c2name, self.third_name.as_ptr());
        if g1 == g2 {
            self.draw_result();
        }
    }


    /// The record of a nation by index (`G_NATION_TABLE` + 0x122 × index): its name (+4) and the name's gender byte.
    unsafe fn nation_by_index(i: i32) -> (usize, usize, i32) {
        let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(0x122));
        (n, n + 4, read_i8(n + nation::GENDER_NAME) as i32)
    }

    /// The club's nation: its short name and the short name's gender byte.
    unsafe fn club_nation_short(&self) -> (usize, i32) {
        let n = self.club_nation();
        (n + nation::SHORT_NAME, read_i8(n + nation::SHORT_GENDER_NAME) as i32)
    }

    /// Kind 9: the media questions the purchase of the player (slot 27: the rival manager's club).
    unsafe fn purchase(&mut self) {
        player_story_piece("purchase (9)");
        covered!("nma_3 purchase");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let c3 = read_usize(G_CLUB_TABLE).wrapping_add((self.slot27 as usize).wrapping_mul(club::SIZE));
        let (c3short, c3g) = (c3 + club::SHORT_NAME, read_i8(c3 + club::SHORT_GENDER_NAME) as i32);
        let rival = self.second_name.as_ptr();
        match self.subtype {
            0 if local => story_lang!(text, 0xa213f4, c3g, c3g, g, g, c3short, rival, name, short),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa212ac, c3g, c3g, 0x64, 0x64, g, g, c3short, rival, self.adjective.as_ptr(), name, short);
            }
            1 => story_lang!(text, 0xa211e0, c3g, c3g, g, g, c3short, rival, short, name),
            2 if local => story_lang!(text, 0xa21124, g, g, name, short),
            2 => {
                self.club_adjective(0x16);
                story_lang!(text, 0xa21048, g, g, 0x64, name, short, self.adjective.as_ptr());
            }
            3 => story_lang!(text, 0xa20f8c, g, g, short, name),
            4 if local => story_lang!(text, 0xa20ec4, 0x64, g, g, name, short),
            4 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa20ddc, 0x64, g, g, 0x64, 0x64, name, short, self.adjective.as_ptr());
            }
            _ if local => story_lang!(text, 0xa20d20, g, g, name, short),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa20c48, ng, ng, g, g, nshort, name, short);
            }
        }
    }

    /// Kind 0x22: the player blasts his manager.
    unsafe fn blast_at_manager(&mut self) {
        player_story_piece("blast at manager (0x22)");
        covered!("nma_3 blast at manager");
        let (short, g) = self.club_short();
        let (text, name, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.third_name.as_ptr());
        if self.subtype != 0 {
            story_lang!(text, 0xa20b08, g, g, short, manager, name, self.third_name5.as_ptr());
        } else {
            story_lang!(text, 0xa209d0, g, g, 0x64, short, name, manager, self.name5.as_ptr());
        }
    }

    /// Kind 0x21: the player blames his manager for his spell of poor form.
    unsafe fn blames_manager(&mut self) {
        player_story_piece("blames manager (0x21)");
        covered!("nma_3 blames manager");
        let (short, g) = self.club_short();
        let (text, name, name5, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa203d0, 0x64, name, manager),
            1 => story_lang!(text, 0xa20494, g, g, 0x64, short, name, manager, name5),
            2 => story_lang!(text, 0xa205dc, 0x64, name, manager, name5, self.third_name5.as_ptr()),
            _ if self.local != 0 => story_lang!(text, 0xa2086c, g, g, 0x64, short, name, name5, manager),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa206e4, g, g, ng, ng, 0x64, short, name, nshort, name5, manager);
            }
        }
    }

    /// Kind 8: the media questions the player's attitude.
    unsafe fn attitude(&mut self) {
        player_story_piece("attitude (8)");
        covered!("nma_3 attitude");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype != 0 {
            story_lang!(text, 0xa202f8, g, mark, mark, g, 0x64, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa20244, g, g, 0x64, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa20174, ng, ng, g, g, 0x64, nshort, name, short);
        }
    }

    /// Kind 1: the player's private life in the papers (slot 7: a nation, slot 8: a city). Subtypes above 6 write
    /// nothing (the original leaves its text buffer as it was).
    unsafe fn private_life(&mut self) {
        player_story_piece("private life (1)");
        covered!("nma_3 private life");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        let slot8 = self.slot8;
        let city = || {
            let c = read_usize(G_CITY_TABLE).wrapping_add((slot8 as usize).wrapping_mul(0x38));
            (c + 4, read_i8(c + city::GENDER_NAME) as i32)
        };
        match self.subtype {
            0 => story_lang!(text, 0xa2006c, mark, mark, g, g, 0x64, outlet, name, short),
            1 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ff4c, mark, mark, 0x64, natg, natg, g, g, outlet, name, nname, short);
            }
            2 if local => story_lang!(text, 0xa1fe8c, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1fdb0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => {
                let (cname, cg) = city();
                story_lang!(text, 0xa1fcd0, cg, cg, name, cname);
            }
            3 => {
                self.club_adjective(0x1d);
                let (cname, cg) = city();
                story_lang!(text, 0xa1fbd4, 0x64, cg, cg, self.adjective.as_ptr(), name, cname);
            }
            4 | 6 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                let tpl = if self.subtype == 4 { 0xa1fb00 } else { 0xa1f8ec };
                story_lang!(text, tpl, mark, mark, g, g, natg, natg, outlet, short, name, nname);
            }
            5 => {
                let (n, _, _) = Self::nation_by_index(self.slot7);
                exe::nation_get_nationality_adjective(n, self.adjective.as_mut_ptr() as usize, 0x78, 2);
                story_lang!(text, 0xa1f9f0, mark, mark, g, g, 0x64, 0x64, outlet, short, name, self.adjective.as_ptr());
            }
            _ => {}
        }
    }

    /// Kind 0x25: the player is concerned by a transfer rumour.
    unsafe fn transfer_rumour(&mut self) {
        player_story_piece("transfer rumour (0x25)");
        covered!("nma_3 transfer rumour");
        let c = self.club_rec();
        let mark = self.mark as i8 as i32;
        let (cname, cg) = (c + club::NAME, read_i8(c + club::GENDER_NAME) as i32);
        let mg = self.mask_gender as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f7b4, mark, mark, 0x64, cg, cg, mg, 0x64, self.outlet_name.as_ptr(), self.name.as_ptr(), cname,
            self.mask_text.as_ptr(), self.fourth_name.as_ptr());
    }

    /// Kind 0x1e: an exclusive article about the player.
    unsafe fn exclusive(&mut self) {
        player_story_piece("exclusive (0x1e)");
        covered!("nma_3 exclusive");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype != 0 {
            if local {
                story_lang!(text, 0xa1f6d8, g, g, name, short);
            } else {
                self.club_adjective(0x19);
                story_lang!(text, 0xa1f5d8, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        } else if local {
            story_lang!(text, 0xa1f4f0, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x19);
            story_lang!(text, 0xa1f3e8, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 7: the player is too young?
    unsafe fn too_young(&mut self) {
        player_story_piece("too young (7)");
        covered!("nma_3 too young");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype == 0 {
            story_lang!(text, 0xa1f1b0, mark, mark, g, g, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa1f334, g, g, short, name);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1f268, ng, ng, g, g, nshort, short, name);
        }
    }

    /// Kind 2: the media raises a question about the player.
    unsafe fn question(&mut self) {
        player_story_piece("question (2)");
        covered!("nma_3 question");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f0f8, mark, mark, 0x64, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x20: rumours about the player (the supporters' club, the press).
    unsafe fn rumours(&mut self) {
        player_story_piece("rumours (0x20)");
        covered!("nma_3 rumours");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa1edf4, g, g, 0x64, short, name),
            1 if self.local != 0 => story_lang!(text, 0xa1efb4, g, g, name, short),
            1 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa1eef0, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1f058, g, g, name, short),
        }
    }

    /// Kind 3: the player is out of favour (slot 7: a nation).
    unsafe fn out_of_favour(&mut self) {
        player_story_piece("out of favour (3)");
        covered!("nma_3 out of favour");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        match self.subtype {
            0 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ecc4, mark, mark, g, g, natg, natg, outlet, name, short, nname);
            }
            1 => story_lang!(text, 0xa1ebd8, mark, mark, g, g, outlet, name, short),
            2 if local => story_lang!(text, 0xa1ead4, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e9b0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => story_lang!(text, 0xa1e8f0, g, g, name, short),
            3 => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e808, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1e72c, mark, mark, g, g, outlet, name, short),
        }
    }

    /// Kind 4: the player is past it?
    unsafe fn past_it(&mut self) {
        player_story_piece("past it (4)");
        covered!("nma_3 past it");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        match self.subtype {
            0 => story_lang!(text, 0xa1e3a4, 0x64, g, g, name, short),
            1 if local => story_lang!(text, 0xa1e504, name),
            1 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e448, ng, ng, nshort, name);
            }
            _ if local => story_lang!(text, 0xa1e678, g, g, name, short),
            _ => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e5a4, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        }
    }


    /// Kind 5: the pressure gets to the player.
    unsafe fn pressure(&mut self) {
        player_story_piece("pressure (5)");
        covered!("nma_3 pressure");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype == 0 {
            if local {
                story_lang!(text, 0xa1e150, g, g, name, short);
            } else {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e084, ng, ng, g, g, nshort, name, short);
            }
        } else if local {
            story_lang!(text, 0xa1e2e0, 0x64, g, g, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1e1fc, ng, ng, 0x64, g, g, nshort, name, short);
        }
    }

    /// Kind 6: the player's indiscipline costs the club dear.
    unsafe fn indiscipline_costs(&mut self) {
        player_story_piece("indiscipline costs (6)");
        covered!("nma_3 indiscipline costs");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1dfb4, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dec0, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 0x10: the player's poor performances and the club's poor run. The foreign text passes the club's gender
    /// byte where the club's name goes (a defect of the original, kept).
    unsafe fn poor_run(&mut self) {
        player_story_piece("poor run (0x10)");
        covered!("nma_3 poor run");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1de14, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dd44, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, g);
        }
    }

    /// Kind 0xa without a match and the kinds without a text: the programmers' placeholder (C runtime sprintf).
    unsafe fn placeholder(&mut self) {
        player_story_piece("placeholder (other kinds)");
        covered!("nma_3 placeholder");
        story_crt!(self.text.as_mut_ptr(), 0xa1dcbc, self.name.as_ptr());
    }

    /// Kinds 0 and 0x12: the media criticises the player. Subtypes 0..7 pick the text; above 7, the bits of slot 18
    /// pick it, and without one the subtype becomes player % 8 (written back to the subtype) and is tried again.
    unsafe fn criticism(&mut self) {
        player_story_piece("criticism (0, 0x12)");
        covered!("nma_3 criticism");
        let (short, g) = self.club_short();
        let (text, name, name5, outlet, manager) =
            (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.outlet_name.as_ptr(), self.third_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let flags = self.slot18 as u32;
        let local = self.local != 0;
        let has = |m: u32| flags & m == m;
        loop {
            match self.subtype as u32 {
                0 if local => story_lang!(text, 0xa1dbe0, g, g, 0x64, short, name),
                0 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1db04, ng, ng, g, g, 0x64, nshort, short, name);
                }
                1 => story_lang!(text, 0xa1da44, mark, mark, 0x64, g, g, outlet, name, short),
                2 => story_lang!(text, 0xa1d98c, g, g, 0x64, short, name),
                3 => story_lang!(text, 0xa1d8c8, g, g, 0x64, short, name),
                4 if local => story_lang!(text, 0xa1d838, g, g, 0x64, short, name),
                4 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1d78c, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, name);
                }
                5 if local => story_lang!(text, 0xa1d690, g, g, 0x64, 0x64, short, manager, name),
                5 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d578, ng, ng, g, g, 0x64, 0x64, nshort, short, manager, name);
                }
                6 if local => story_lang!(text, 0xa1d470, g, g, 0x64, short, manager, name),
                6 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d34c, ng, ng, g, g, 0x64, nshort, short, manager, name);
                }
                7 if local => story_lang!(text, 0xa1d20c, g, g, 0x64, short, manager, name),
                7 => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1d0a8, 0x64, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, manager, name);
                }
                _ if has(0x400200) => story_lang!(text, 0xa1cfd4, mark, mark, g, g, 0x64, outlet, name, short),
                _ if has(0x102200) => story_lang!(text, 0xa1ce94, mark, mark, 0x64, g, g, 0x64, outlet, name, short, name5),
                _ if has(0x104200) => story_lang!(text, 0xa1cd30, mark, mark, 0x64, g, g, outlet, name, short, name5),
                _ if has(0x100200) => story_lang!(text, 0xa1cc68, mark, mark, 0x64, g, g, outlet, name, short),
                _ if has(0x480) => story_lang!(text, 0xa1cb84, mark, mark, g, g, outlet, name, short),
                _ if has(0x500) => story_lang!(text, 0xa1c9ec, g, g, mark, mark, short, outlet, name),
                _ if has(0x20820) => story_lang!(text, 0xa1c8e8, 0x64, g, g, mark, mark, name, short, outlet),
                _ if has(0x1020) => story_lang!(text, 0xa1c7b4, 0x64, g, g, mark, mark, 0x64, name, short, outlet, name5),
                _ if has(0x20810) && local => story_lang!(text, 0xa1c6bc, 0x64, g, g, name, short),
                _ if has(0x20810) => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1c5b0, 0x64, g, g, 0x64, name, short, self.adjective.as_ptr());
                }
                _ if has(0x10001) && local => story_lang!(text, 0xa1c480, g, g, short, name, name5),
                _ if has(0x10001) => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1c334, g, g, ng, ng, short, name, nshort, name5);
                }
                _ if flags & 1 != 0 && local => story_lang!(text, 0xa1c204, g, g, short, name, name5),
                _ if flags & 1 != 0 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1c0b8, g, g, 0x64, short, name, self.adjective.as_ptr(), name5);
                }
                _ if flags as u8 & 0xa == 0xa => story_lang!(text, 0xa1bf7c, g, g, mark, mark, short, name, outlet, name5),
                _ if flags as u8 & 8 == 8 && local => story_lang!(text, 0xa1bed8, g, g, short, name),
                _ if flags as u8 & 8 == 8 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1be08, g, g, ng, ng, short, name, nshort);
                }
                _ if flags as u8 & 2 == 2 && local => story_lang!(text, 0xa1bcf8, g, g, 0x64, short, name, name5),
                _ if has(0x4000) => story_lang!(text, 0xa1bc50, g, g, short, name),
                _ if has(0x20000) => story_lang!(text, 0xa1bba0, g, g, short, name),
                _ if has(0x40000) && flags as u8 & 0x30 == 0 => story_lang!(text, 0xa1ba44, g, g, short, name, name5),
                _ => {
                    self.subtype = self.player % 8; // the C remainder (a negative player would loop for ever, as in the original)
                    continue;
                }
            }
            return;
        }
    }


    /// The kind dispatch of the story text (kind = slot 9).
    unsafe fn write_story(&mut self) {
        match self.kind {
            0x23 => self.left_out(),
            0xe => self.ability(),
            0xd => self.complacency(),
            0x11 => self.contribution(),
            0 | 0x12 => self.criticism(),
            0xc => self.commitment(),
            0x13 => self.indiscipline(),
            0xa if self.with_fixture != 0 => self.contribution_after_match(),
            9 => self.purchase(),
            0x22 => self.blast_at_manager(),
            0x21 => self.blames_manager(),
            8 => self.attitude(),
            1 => self.private_life(),
            0x25 => self.transfer_rumour(),
            0x1e => self.exclusive(),
            7 => self.too_young(),
            2 => self.question(),
            0x20 => self.rumours(),
            3 => self.out_of_favour(),
            4 => self.past_it(),
            5 => self.pressure(),
            6 => self.indiscipline_costs(),
            0x10 => self.poor_run(),
            _ => self.placeholder(),
        }
    }

    /// Kinds 0 and 0x12 with a subtype above 7 (picked by the bits of slot 18): the bits of a "bad" story.
    fn bad_flags(&self) -> bool {
        let f = self.slot18 as u32;
        [0x400200, 0x102200, 0x104200, 0x100200, 0x480, 0x500, 0x20820, 0x1020, 0x20810, 0x10001].iter().any(|&m| f & m == m) || f & 1 != 0
    }

    /// Kinds 0 and 0x12 with a subtype above 7, without the bad bits: the bits of a player coping with the pressure.
    fn coping_flags(&self) -> bool {
        let f = self.slot18 as u32;
        f as u8 & 0xa == 0xa || f as u8 & 8 == 8 || (f as u8 & 2 == 2 && self.local != 0)
    }

    /// Kinds 0 and 0x12 with a subtype above 7.
    fn criticism_by_flags(&self) -> bool {
        (self.kind == 0 || self.kind == 0x12) && self.subtype > 7
    }

    /// The parity of the subtype (C remainder by 2).
    fn subtype_even(&self) -> bool {
        self.subtype % 2 == 0
    }

    /// The reply about the player's role in the squad (slot 16: the role, slot 17: the age), by the human manager
    /// (`extra`, with the player's name in form 5).
    unsafe fn human_squad_role(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        match self.slot16 {
            1..=3 => story_lang!(extra, 0xa1af60),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa1b14c);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1b074, name5);
                } else {
                    story_lang!(extra, 0xa1b0d0, 0x64, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa1b008, name5),
            4 | 5 => story_lang!(extra, 0xa1aefc, name5),
            6 => story_lang!(extra, 0xa1ae98, name5),
            _ => story_lang!(extra, 0xa1b0d0, 0x64, name5),
        }
    }

    /// The same reply by the player's club manager (the third person, slot 23).
    unsafe fn club_manager_squad_role(&mut self) {
        let (extra, name5, manager) = (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.slot16 {
            1 => story_lang!(extra, 0xa19c1c, manager, name5),
            2 | 3 => story_lang!(extra, 0xa19b94, manager, name5),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa19b10, manager, name5);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1996c, manager, name5);
                } else {
                    story_lang!(extra, 0xa19a70, 0x64, manager, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa198d8, manager, name5),
            4 | 5 => story_lang!(extra, 0xa1984c, manager, name5),
            6 => story_lang!(extra, 0xa197c4, manager, name5),
            _ => story_lang!(extra, 0xa19a70, 0x64, manager, name5),
        }
    }

    /// The paragraph after the story (`extra`, slot 10 = the reply): when the third person is the human manager on
    /// screen, his reply (1 support, 2 criticise, 3 ignore); otherwise the club manager's (1 support, 2 criticise).
    unsafe fn write_reply(&mut self) {
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        let extra = self.extra.as_mut_ptr();
        if read_usize(G_STAFF_TABLE).wrapping_add(self.third_off) == manager {
            player_story_piece("manager reply");
            covered!("nma_3 manager reply");
            match self.reader_case {
                1 => self.human_supports(),
                2 => self.human_criticises(),
                3 => story_lang!(extra, 0xa1ba14), // "You have decided to ignore this news report."
                _ => story_crt!(extra, 0xad9c64),
            }
        } else {
            player_story_piece("club manager reply");
            covered!("nma_3 club manager reply");
            match self.reader_case {
                1 => self.club_manager_supports(),
                2 => self.club_manager_criticises(),
                _ => story_crt!(extra, 0xad9c64),
            }
        }
    }

    /// The human manager criticises the player (reply 2).
    unsafe fn human_criticises(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1b958);
            } else {
                story_lang!(extra, 0xa1b9b4, 0x64, name5);
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1b8e4),
            0x11 => story_lang!(extra, 0xa1b86c, 0x64, name5),
            0 => story_lang!(extra, 0xa1b9b4, 0x64, name5),
            0xe => story_lang!(extra, 0xa1b81c),
            0xc | 0xd => story_lang!(extra, 0xa1b18c, 0x64, name5),
            0x13 => story_lang!(extra, 0xa1b79c, name5),
            0xa | 0x21 | 0x22 => story_lang!(extra, 0xa1b6ec, name5),
            8 => story_lang!(extra, 0xa1b674),
            1 => story_lang!(extra, 0xa1b608, name5),
            2 | 9 => story_lang!(extra, 0xa1b1fc, 0x64, name5),
            0x25 => story_lang!(extra, 0xa1b5c8),
            0x1e | 0x20 => story_lang!(extra, 0xa1b268, 0x64, name5),
            3 => story_lang!(extra, 0xa1b56c, 0x64, name5),
            4 => {
                let (short, g) = self.club_short();
                let tpl = if self.slot14_again != 0 { 0xa1b4c0 } else { 0xa1b414 };
                story_lang!(extra, tpl, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1b2dc, name5),
            6 => story_lang!(extra, 0xa1b378, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The human manager supports the player (reply 1).
    unsafe fn human_supports(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1afa4);
            } else {
                self.human_squad_role();
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1ae44),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1ad78, name5),
            0x13 => story_lang!(extra, 0xa1acdc, name5),
            0xa => story_lang!(extra, 0xa1ac78, name5),
            0xc | 0xd => story_lang!(extra, 0xa1a6ac),
            0xe => story_lang!(extra, 0xa1ac38),
            0x11 => story_lang!(extra, 0xa1abcc, 0x64, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa1ab6c),
            0 => self.human_squad_role(),
            0x21 | 0x22 => story_lang!(extra, 0xa1a714, name5),
            8 => story_lang!(extra, 0xa1ab14),
            1 => story_lang!(extra, 0xa1aabc, name5),
            2 | 9 => story_lang!(extra, 0xa1a77c, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa1aa18, mg, 0x64, self.mask_text.as_ptr(), name5);
            }
            0x1e | 0x20 => story_lang!(extra, 0xa1a7e0),
            3 => story_lang!(extra, 0xa1a9b8, name5),
            4 => {
                let (short, g) = self.club_short();
                story_lang!(extra, 0xa1a920, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1a81c, name5),
            6 => story_lang!(extra, 0xa1a8a0, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The club manager criticises the player (reply 2).
    unsafe fn club_manager_criticises(&mut self) {
        let (extra, name, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1a5ec, manager);
            } else {
                story_lang!(extra, 0xa1a65c, manager);
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1a580, manager),
            0x11 => story_lang!(extra, 0xa1a4e8, 0x64, manager, name5),
            0 | 2 | 9 => story_lang!(extra, 0xa1a65c, manager),
            0xc | 0xd => story_lang!(extra, 0xa19ca0, 0x64, manager, name5),
            8 => story_lang!(extra, 0xa1a440, manager, name),
            0x13 if self.byte_3271 != 0 => story_lang!(extra, 0xa1a3a8, manager5, name5),
            0x13 => story_lang!(extra, 0xa1a310, manager5, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa19d2c, manager5, name5),
            1 => story_lang!(extra, 0xa1a254, g, g, short, manager, name5),
            0x25 => story_lang!(extra, 0xa1a164, manager),
            0x1e | 0x20 => story_lang!(extra, 0xa19e14, manager),
            3 => story_lang!(extra, 0xa1a0dc, manager, name5),
            4 => story_lang!(extra, 0xa1a014, g, g, manager, name5, short),
            6 => story_lang!(extra, 0xa19f64, 0x64, manager, name5),
            5 | 7 => story_lang!(extra, 0xa19e80, g, g, short, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The club manager supports the player (reply 1).
    unsafe fn club_manager_supports(&mut self) {
        let (extra, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa199ec, manager);
            } else {
                self.club_manager_squad_role();
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1975c, manager),
            0x11 => story_lang!(extra, 0xa196d0, 0x64, manager, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa19644, manager),
            0 => self.club_manager_squad_role(),
            0xc | 0xd => story_lang!(extra, 0xa18e44, manager, name5),
            8 => story_lang!(extra, 0xa195e0, manager),
            1 => story_lang!(extra, 0xa19560, manager, name5),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1947c, name5),
            0x13 => story_lang!(extra, 0xa193c8, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa18eec, manager5, name5),
            2 | 9 => story_lang!(extra, 0xa18f70, manager, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa19324, mg, 0x64, manager, self.mask_text.as_ptr());
            }
            0x1e | 0x20 => story_lang!(extra, 0xa18ffc, manager),
            3 => story_lang!(extra, 0xa192a0, manager, name5),
            4 => story_lang!(extra, 0xa191e4, g, g, manager, name5, short),
            5 | 7 => story_lang!(extra, 0xa19060, g, g, short, manager, name5),
            6 => story_lang!(extra, 0xa19128, 0x64, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The links of a person when his record matches the names (`exe::database_staff_matches_names`): one per text
    /// given, type 0x7d7, the person's record.
    unsafe fn link_person(out: usize, index: i32, names: [i32; 3], texts: &[usize]) {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((index as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
            for &t in texts {
                exe::text_add_link(out, t as *const u8, 0x7d7, rec);
            }
        }
    }

    /// The body's text ("%s\n\n%s": the story and the reply) and its links: the persons, the club's nation (name,
    /// short name and the adjectives of forms 0x16..0x1f, to the national team), the club, and after a match the score
    /// (to the fixture, when `Match_Stats::fixture_entry_matches` finds it) and the other club.
    unsafe fn finish(&mut self, out: usize) {
        player_story_piece("links");
        covered!("nma_3 links");
        story_crt!(self.long_extra.as_mut_ptr(), 0x991948, self.text.as_ptr(), self.extra.as_ptr());
        exe::text_set_string(out, self.long_extra.as_ptr());
        if self.second > -1 {
            Self::link_person(out, self.second, self.second_names, &[self.second_name.as_ptr() as usize]);
        }
        if self.player != -1 {
            Self::link_person(out, self.player, self.names, &[self.name.as_ptr() as usize, self.name5.as_ptr() as usize]);
        }
        if self.fourth != -1 {
            Self::link_person(out, self.fourth, self.fourth_names, &[self.fourth_name.as_ptr() as usize]);
        }
        if self.third != -1 {
            Self::link_person(out, self.third, self.third_names, &[self.third_name5.as_ptr() as usize, self.third_name.as_ptr() as usize]);
        }
        if self.club != -1 {
            let team = |st: &PlayerStory| exe::database_get_national_team(st.club_nation());
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + 4) as *const u8, 0x7d5, t);
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + nation::SHORT_NAME) as *const u8, 0x7d5, t);
            for form in 0x16..=0x1f {
                self.club_adjective(form);
                let t = team(self);
                exe::text_add_link(out, self.adjective.as_ptr(), 0x7d5, t);
            }
            let c = self.club_rec();
            exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
            exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);
        }
        if self.with_fixture == 0 {
            return;
        }
        let d = |k: usize| u32::from_le_bytes([self.fixture[k], self.fixture[k + 1], self.fixture[k + 2], self.fixture[k + 3]]);
        let matches: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(PLAYER_STORY_FIXTURE_ENTRY_MATCHES);
        if matches(read_usize(G_MATCH_STATS), d(0x1b), d(0x11), d(0x15), d(0), d(2)) != 0 {
            exe::text_add_link(out, self.score.as_ptr(), 0x7d3, d(0x1b) as usize);
        }
        let c2 = self.other_club();
        exe::text_add_link(out, (c2 + club::NAME) as *const u8, 0x7d5, c2);
    }
}

/// The headline (`body` 0): by the story kind (slot 9), with the player's name in form 5 (slots 0..3) and the second
/// person's (slots 11..14); links: the player, the second person, the club (short name and name) and its nation (to
/// the national team).
unsafe fn player_story_headline(news: usize, out: usize) {
    player_story_piece("headline");
    covered!("nma_3 headline");
    let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
    let player = s(0);
    let names = [s(1), s(2), s(3)];
    let club = s(4);
    let subtype = s(5);
    s(6);
    s(6); // the original asks for slot 6 twice and never for 7 and 8
    let kind = s(9);
    s(10);
    let second = s(11);
    let second_names = [s(12), s(13), s(14)];
    s(18);
    s(23);
    s(24);
    s(25);
    s(26);
    let mut mask_text = [0u8; 0x200];
    let mask = s(0x20);
    let mask_text_fn: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
    mask_text_fn(mask as u32, mask_text.as_mut_ptr() as usize, 1);
    let c = read_usize(G_CLUB_TABLE).wrapping_add((club as usize).wrapping_mul(club::SIZE));
    let mut name = [0u8; 0x200];
    let mut second_name = [0u8; 0x200];
    let mut text = [0u8; 0x1388];
    // the size given (0x190) is larger than the original's buffers (0xc8 and 0x78)
    exe::database_format_name_from_ids(names[0] as u32, names[1] as u32, names[2] as u32, name.as_mut_ptr(), 0x190, 5, player as u32);
    if second != -1 {
        exe::database_format_name_from_ids(second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, second_name.as_mut_ptr(), 0x190, 5, player as u32);
    } else {
        second_name[0] = 0;
    }
    let (t, n) = (text.as_mut_ptr(), name.as_ptr());
    let (short, g) = (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32);
    match kind {
        0x23 => story_lang!(t, 0xa18e14, n),
        0xe => story_lang!(t, 0xa18dd4, 0x64, n),
        4 => story_lang!(t, 0xa18da4, n),
        0xd => story_lang!(t, 0xa18d64, n),
        0x11 => match (club / (subtype + 1) + player) % 3 {
            0 => story_lang!(t, 0xa18c94, 0x64, n),
            1 => story_lang!(t, 0xa18cdc, n),
            _ => story_lang!(t, 0xa18d18, 0x64, n),
        },
        0 => match subtype % 3 {
            0 => story_lang!(t, 0xa18c60, n),
            1 => story_lang!(t, 0xa18c2c, n),
            _ => story_lang!(t, 0xa18bf8, n),
        },
        9 if subtype == 0 || subtype == 1 => story_lang!(t, 0xa18b50, 0x64, second_name.as_ptr(), n),
        9 => story_lang!(t, 0xa18bb4, 0x64, n),
        0x13 | 0x21 | 0xa => story_lang!(t, 0xa188d8, g, g, short),
        0x22 => story_lang!(t, 0xa18b18, n),
        8 => story_lang!(t, 0xa18ad4, 0x64, n),
        3 => story_lang!(t, 0xa18aa0, n),
        5 => story_lang!(t, 0xa18a68, n),
        7 => story_lang!(t, 0xa18a34, n),
        6 => story_lang!(t, 0xa189d0, 0x64, g, g, n, short),
        0xc => story_lang!(t, 0xa18990, 0x64, n),
        0x25 => story_lang!(t, 0xa18950, n),
        _ => story_lang!(t, 0xa18910, 0x64, n),
    }
    exe::text_set_string(out, text.as_ptr());
    let rec = read_usize(G_STAFF_TABLE).wrapping_add((player as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, rec);
    }
    if second > -1 {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((second as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, 0usize) != 0 {
            exe::text_add_link(out, second_name.as_ptr(), 0x7d7, rec);
        }
    }
    if club == -1 {
        return;
    }
    exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
    exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);
    let n = read_usize(c + club::NATION);
    let t = exe::database_get_national_team(n);
    exe::text_add_link(out, (read_usize(c + club::NATION) + 4) as *const u8, 0x7d5, t);
}

/// `Media::write_player_media_story_news_text(news, body, out)`: writes the headline or the body of a media story about
/// a player into the text `out`.
///
/// The headline (`body` 0) is one line by the story kind (slot 9). The body reads the slots (0: the player, 1..3 his
/// names, 4: his club, 5: the subtype, 6: the outlet type, 9: the kind, 10: the reply, 11..14: a second person, 16..18:
/// the squad role, the age and the story bits, 23..26: the third person, the club manager or the human manager, 28:
/// with a match, then the fixture record and the result, or the qualities, the position, the pundit and a fourth
/// person) and writes, in pieces: the outlet's name (the pundit's when the media is local, else the outlet type, plain
/// for the local media or with the adjective of the club's nation), the story by kind (left out, ability, complacency,
/// contribution with the score, criticism by the bits of slot 18, commitment, indiscipline, purchase, the manager rows,
/// attitude, private life, transfer rumour, exclusive, too young, rumours, out of favour, past it, pressure, poor run;
/// the other kinds the programmers' placeholder), how a drawn match was eventually won or lost, then the reply
/// paragraph: the human manager's when he is the third person (support, criticise, ignore), otherwise the club
/// manager's; the text is "%s\n\n%s" of the two, with the links of the persons whose names match, the club's nation
/// (name, short name, ten adjectives), the club, the score and the other club.
///
/// Defects of the original kept: slot 14 is read twice and 15 never; the kind 0x10 foreign text passes the club's
/// gender byte where the club's name goes; the kind 1 story above subtype 6 writes nothing (the original then prints its
/// uninitialised buffer; the copy an empty text); the headline's names get a size (0x190) larger than their buffers;
/// the headline of kind 0x11 divides by subtype + 1 (a subtype of -1 divides by zero, in both).
/// Original: `Media::write_player_media_story_news_text` (0x72d270, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_media_story_news_text(_this: usize, news: usize, body: u32, out: usize) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().clear());
    if body == 0 {
        return player_story_headline(news, out);
    }
    let mut st = PlayerStory::read(news);
    st.write_outlet();
    st.write_story();
    st.write_reply();
    st.finish(out);
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc).
unsafe fn nma3_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
    let text = |p: usize| if p == 0 { Vec::new() } else { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec() };
    let mut links = Vec::new();
    let l = read_usize(t + 4);
    if l != 0 {
        list_rewind(l);
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            links.push((text(e), read_u32(e + 0xc8), read_u32(e + 0xcc)));
        }
    }
    (text(read_usize(t)), links)
}

/// Comparer for `write_player_media_story_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator's state, without counting); the strings and the links (text, type and data) must
/// be equal. The message names the last piece the copy ran (headline, outlet, the story kind, the draw result, the
/// reply, the links).
pub(crate) unsafe extern "thiscall" fn compare_write_player_media_story_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_PLAYER_MEDIA_STORY_NEWS_TEXT);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let mut pieces = Vec::new();
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out), || {
        crate::rng::without_counting(|| {
            write_player_media_story_news_text(this, news, body, lp);
            pieces = PLAYER_STORY_TRAIL.with(|t| t.borrow().clone());
        })
    });
    let (mo, mr) = (nma3_text_contents(out), nma3_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || {
        format!("write_player_media_story_news_text(body {}) pieces {:?}; original {:?} {:x?} rust {:?} {:x?}", body, pieces,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}

// ---------- batch 294: the player praise story (media.cpp) ----------

/// Where each text buffer of `write_player_praise_news_text` lives in its frame (the original's `esp` offsets after the
/// three register pushes; the copy keeps the same layout, so a long text overflows the same way).
#[allow(dead_code)]
mod praise_frame {
    pub(crate) const POSITION: usize = 0xe4; // the position name or player description (0xc8)
    pub(crate) const ADJECTIVE: usize = 0x1b8; // a nationality adjective (0x78)
    pub(crate) const PLAYER_FULL: usize = 0x230; // the player's name, style 1 (0x168)
    pub(crate) const LEGEND: usize = 0x398; // the great player's name (0x78)
    pub(crate) const PLAYER_SHORT: usize = 0x410; // the player's name, style 5 (0x168)
    pub(crate) const EXTRA: usize = 0x578; // the stage label of kind 0x34 (0xc8)
    pub(crate) const OUTLET: usize = 0x640; // the outlet or pundit name (0xc8)
    pub(crate) const REPLY: usize = 0x708; // the second paragraph: the manager's reply (0x3e8)
    pub(crate) const MANAGER_FULL: usize = 0xaf0; // the manager's name, style 1 (0x168)
    pub(crate) const DATE_TEXT: usize = 0xc58; // a date text (0x78)
    pub(crate) const HEAD: usize = 0xcd0; // the headline, or the body's first paragraph
    pub(crate) const SPLIT_EXT: usize = 0x2058; // the source file's extension, for the error box (0x100)
    pub(crate) const SPLIT_NAME: usize = 0x2158; // the source file's name, for the error box (0x100)
    pub(crate) const MANAGER_SHORT: usize = 0x2258; // the manager's name, style 5 (0x168)
    pub(crate) const SIZE: usize = 0x23c0;
    /// the three great players named when the story has none (by (variant + player) % 4)
    pub(crate) const LEGENDS: [usize; 4] = [0xa434f0, 0xa43500, 0xa43510, 0xa43518]; // Charlton, Maradona, Pelé, Cruyff
}

/// The news slots and the frame of `write_player_praise_news_text`. The slots after 17 are read from a moving index
/// (0x12, or after the match record when slot 17 is set); the names below are those of the use, [inference] where only
/// the use says so.
struct Praise {
    /// the frame's buffers (`praise_frame`), on the caller's stack
    base: usize,
    /// slot 0: the player (staff index); 1-3 his name ids
    player: i32,
    names: [u32; 3],
    /// slot 4: the player's club (club index); `club` is its record (club table + index × `club::SIZE`, unchecked)
    club_index: i32,
    club: usize,
    /// slot 5: the story kind (0x29..0x34)
    kind: i32,
    /// slot 7: a nation linked at the end (index)
    nation_a: i32,
    /// slot 8: the variant (picks the paragraph)
    variant: i32,
    /// slot 9: the manager who replies (staff index); 10-12 his name ids
    manager: i32,
    manager_names: [u32; 3],
    /// slot 13: the reply (1, 2, 3 no reply)
    response: i32,
    /// slot 14: the player's strength flags; slot 15: his position flags
    strengths: u32,
    position: u32,
    /// slot 16: a competition linked at the end (club comp index)
    comp: i32,
    /// slot 17: the story carries a match record (read by `read_match_record_from_news`)
    has_match: u32,
    /// the moving slots: a rival club, four values, the great player and his name ids, two nations, a value, the great
    /// player's job and club, the outlet type and pundit, the player's age [inference] and a flag
    rival_club: i32,
    m1: i32,
    m2: i32,
    m3: i32,
    m4: i32,
    legend: i32,
    legend_names: [u32; 3],
    nation_b: i32,
    nation_c: i32,
    m11: i32,
    legend_job: i32,
    legend_club: i32,
    outlet_type: i32,
    pundit: i32,
    age: i32,
    m17: i32,
    /// the human manager's (screen manager's) club or nation is the player's club's nation: "national" press, else
    /// "{nationality} national" press
    home: bool,
    /// the gender byte of the position text (the dword at frame +0x10 is what the original passes on)
    pos_gender: u32,
    /// the outlet's gender byte, the great player's job gender byte
    outlet_gender: u8,
    job_gender: u8,
    /// the match record read from the news (the original's frame +0x6c)
    record: [u8; 0x20],
    /// the name of the piece that wrote the text (for the comparer)
    piece: &'static str,
}

type PraiseFormat = unsafe extern "C" fn(usize, usize, ...) -> i32;

impl Praise {
    unsafe fn format() -> PraiseFormat {
        core::mem::transmute(FN_LANGLIB_SPRINTF)
    }
    fn at(&self, off: usize) -> usize {
        self.base + off
    }
    /// the club's short-name gender byte (signed) and short name
    unsafe fn cg(&self) -> i32 {
        read_i8(self.club + club::SHORT_GENDER_NAME) as i32
    }
    fn cname(&self) -> usize {
        self.club + club::SHORT_NAME
    }
    /// the club's long-name gender byte and long name
    unsafe fn lg(&self) -> i32 {
        read_i8(self.club + club::GENDER_NAME) as i32
    }
    fn lname(&self) -> usize {
        self.club + club::NAME
    }
    unsafe fn nation(&self) -> usize {
        read_usize(self.club + club::NATION)
    }
    /// the position text's gender byte, signed
    fn pg(&self) -> i32 {
        self.pos_gender as u8 as i8 as i32
    }
    /// `write_player_description(player, position, strengths, POSITION, 0xc8, gender, 1)` (0x5465a0, cdecl; the
    /// original pushes a 7th argument, 1, that the callee does not read): the description over the position text,
    /// returning its gender byte (signed)
    unsafe fn describe(&mut self) -> i32 {
        let f: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(0x5465a0usize);
        let out = self.at(praise_frame::POSITION);
        f(self.player, self.position, self.strengths, out, 0xc8, self.pos_gender, 1) as i8 as i32
    }
    /// the description, keeping its gender byte as the position gender (the original stores al at frame +0x10)
    unsafe fn describe_keep(&mut self) {
        let g = self.describe();
        self.pos_gender = (self.pos_gender & 0xffff_ff00) | (g as u8 as u32);
    }
    /// the nationality adjective of the club's nation in `form` into ADJECTIVE (0x78)
    unsafe fn adjective(&mut self, form: u32) {
        let out = self.at(praise_frame::ADJECTIVE);
        exe::nation_get_nationality_adjective(self.nation(), out, 0x78, form);
    }
    /// `database_get_job_name(job, 0, 0, &gender)` of the great player's job (0x53ad00, cdecl)
    unsafe fn legend_job_name(&mut self) -> usize {
        let f: unsafe extern "C" fn(i32, u32, u32, *mut u8) -> usize = core::mem::transmute(0x53ad00usize);
        f(self.legend_job, 0, 0, &mut self.job_gender)
    }
    fn jg(&self) -> i32 {
        self.job_gender as i8 as i32
    }
    /// the great player's club record and its short-name gender byte
    unsafe fn legend_club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.legend_club as usize).wrapping_mul(club::SIZE))
    }
    /// the great player's nation (staff `NATION` of his record)
    unsafe fn legend_nation(&self) -> usize {
        read_usize(read_usize(G_STAFF_TABLE).wrapping_add((self.legend as usize).wrapping_mul(staff::SIZE)) + staff::NATION)
    }
    /// the first paragraph (or the headline) and the reply
    fn head(&self) -> usize {
        self.at(praise_frame::HEAD)
    }
    fn reply(&self) -> usize {
        self.at(praise_frame::REPLY)
    }
    fn pos(&self) -> usize {
        self.at(praise_frame::POSITION)
    }
    fn full(&self) -> usize {
        self.at(praise_frame::PLAYER_FULL)
    }
    fn short(&self) -> usize {
        self.at(praise_frame::PLAYER_SHORT)
    }
    fn adj(&self) -> usize {
        self.at(praise_frame::ADJECTIVE)
    }
    fn lname_buf(&self) -> usize {
        self.at(praise_frame::LEGEND)
    }
}

/// `Media::write_player_praise_news_text(news, body, out)`: writes the headline or the body of a media story that
/// praises a player, naming the outlet or pundit, a great player, the player and the manager's reply, then links every
/// name in the text.
///
/// Reads the news slots (`exe::newsitem_get_slot`): 0 the player (staff index) and 1-3 his name ids, 4 his club, 5 the
/// kind (0x29..0x34), 7 a nation, 8 the variant, 9 the manager and 10-12 his name ids, 13 the reply, 14 the strength
/// flags, 15 the position flags, 16 a competition, 17 "with a match record"; then, from slot 0x12 or after the match
/// record (`read_match_record_from_news` 0x7e4fa0, which moves the index), a rival club, four values, the great player
/// and his name ids, two nations, a value, the great player's job and club (−1 out of range), the outlet type and
/// pundit, the player's age [inference] and a flag. With a human manager on screen (`G_SCREEN_RECORDS_MANAGER` of
/// `G_SCREEN_MANAGER_CURRENT_SLOT`), the rival club is dropped when values 1 and 4 are 0, the player % 4 is not 0 and
/// the rival is of the manager's nation (his club job's nation, or his own); `home` is set when the player's club is
/// of that nation. The position text (`write_position_name` of the position flags, 0xc8) gives the gender; when the
/// manager is the player himself it reads "player-manager" (gender 0).
///
/// The outlet (`praise_outlet`): the pundit's name (`PunditList::get_pundit_name` of `G_PUNDIT_LIST`) at home with a
/// nation, an outlet type and a pundit; otherwise by type 1..6 and other: national or local newspaper, national or
/// local radio station, magazine, sports paper, website ("{nationality} ..." abroad, `nation_get_nationality_adjective`
/// forms 0x16..0x1a; the magazine's gender is 1 with language 8, the others 0). The great player: his name from the ids,
/// or Bobby Charlton, Diego Maradona, Pelé or Johan Cruyff by (variant + player) % 4. The names of the player and the
/// manager in styles 1 and 5 (`database_format_name_from_ids`, 0x168).
///
/// Body: the first paragraph by kind (one piece each, `praise_body_*`), then the reply (`praise_reply`), joined as
/// "%s\n\n%s" in the text buffer 0xde1f64. Headline (`praise_headline`): "Praise for {club} {position}" when variant % 8
/// is player % 7, otherwise by kind. The text goes to `out` (`exe::text_set_string`) and the links follow
/// (`praise_links`): the match (0x7d3) and its club, the player, the manager and the great player when their ids match
/// (`database_staff_matches_names`, 0x7d7), the great player's club, the rival club and the player's club (0x7d5), the
/// club's nation and its national team under ten adjective forms (0x16..0x1f), three nations and the competition
/// (0x7d0).
///
/// Defect of the original: the match link's text is the frame byte +0x4c, zeroed only in the headline; in the body it is
/// whatever the stack held (the copy writes an empty text in both, and the comparer does not compare that text).
/// Original: `Media::write_player_praise_news_text` (0x744f80, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_praise_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut frame = [0u8; praise_frame::SIZE];
    let mut p = praise_read_slots(news, frame.as_mut_ptr() as usize);
    praise_setup(&mut p);
    praise_outlet(&mut p);
    praise_names(&mut p);
    let text = if body != 0 {
        covered!("nma_4 praise body");
        praise_body(&mut p);
        praise_reply(&mut p);
        write_u8(PRAISE_TEXT_BUFFER, 0);
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(PRAISE_TEXT_BUFFER, 0x991948, p.head(), p.reply()); // "%s\n\n%s"
        PRAISE_TEXT_BUFFER
    } else {
        praise_headline(&mut p);
        p.head()
    };
    exe::text_set_string(out, text as *const u8);
    praise_links(&mut p, out);
    PRAISE_LAST_PIECE = p.piece;
}

/// The text buffer the body is joined in (`g_text_buffer`).
const PRAISE_TEXT_BUFFER: usize = 0xde1f64;

/// The piece of the last text written (for the comparer's message).
static mut PRAISE_LAST_PIECE: &str = "";

/// Reads the news slots in the original's order (the match record moves the index of the slots after 17).
unsafe fn praise_read_slots(news: usize, base: usize) -> Praise {
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let player = s(0) as i32;
    let names = [s(1), s(2), s(3)];
    let club_index = s(4) as i32;
    let kind = s(5) as i32;
    s(6);
    let nation_a = s(7) as i32;
    let variant = s(8) as i32;
    let manager = s(9) as i32;
    let manager_names = [s(10), s(11), s(12)];
    let response = s(13) as i32;
    let strengths = s(14);
    let position = s(15);
    let comp = s(16) as i32;
    let has_match = s(17);
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let mut k: u8 = 0x12;
    if has_match != 0 {
        let f: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> i32 = core::mem::transmute(0x7e4fa0usize);
        f(news, &mut k, record.as_mut_ptr());
    }
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as u32);
        k = k.wrapping_add(1);
        v
    };
    let rival_club = next() as i32;
    let (m1, m2, m3, m4) = (next() as i32, next() as i32, next() as i32, next() as i32);
    let legend = next() as i32;
    let legend_names = [next(), next(), next()];
    let (nation_b, nation_c, m11) = (next() as i32, next() as i32, next() as i32);
    let legend_job = next() as i32;
    let mut legend_club = next() as i32;
    let (outlet_type, pundit, age, m17) = (next() as i32, next() as i32, next() as i32, next() as i32);
    if legend_club < 0 || legend_club >= read_i32(G_CLUB_COUNT) {
        legend_club = -1;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE));
    Praise {
        base, player, names, club_index, club, kind, nation_a, variant, manager, manager_names, response, strengths,
        position, comp, has_match, rival_club, m1, m2, m3, m4, legend, legend_names, nation_b, nation_c, m11,
        legend_job, legend_club, outlet_type, pundit, age, m17, home: false, pos_gender: 0, outlet_gender: 0,
        job_gender: 0, record, piece: "",
    }
}

/// The C remainder of `a % b` (signed, the sign of `a`), as the original's `idiv` and `and 0x80000003` fix-ups give.
fn praise_rem(a: i32, b: i32) -> i32 {
    a.wrapping_rem(b)
}

/// The human manager's nation test (drops the rival club, sets `home`) and the position text.
unsafe fn praise_setup(p: &mut Praise) {
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human != 0 {
        let club_nation = |c: i32| read_usize(read_usize(G_CLUB_TABLE).wrapping_add((c as usize).wrapping_mul(club::SIZE)) + club::NATION);
        let job = read_usize(human + staff::CLUB_JOB);
        let mine = if job != 0 { read_usize(job + club::NATION) } else { read_usize(human + staff::NATION) };
        if p.m1 == 0 && p.m4 == 0 && p.rival_club > -1 && praise_rem(p.player, 4) != 0 && club_nation(p.rival_club) == mine {
            covered!("nma_4 praise rival dropped");
            p.rival_club = -1;
        }
        if club_nation(p.club_index) == mine {
            p.home = true;
        }
    }
    let f: unsafe extern "C" fn(u32, usize, i32, u32) -> u8 = core::mem::transmute(0x5461b0usize);
    p.pos_gender = f(p.position, p.pos(), 0xc8, 1) as u32;
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    if staff_of(p.manager) == staff_of(p.player) {
        Praise::format()(p.pos(), 0xa242e0); // "player-manager"
        p.pos_gender &= 0xffff_ff00;
    }
}

/// The outlet's name and gender byte (see the routine's header).
unsafe fn praise_outlet(p: &mut Praise) {
    let f = Praise::format();
    let out = p.at(praise_frame::OUTLET);
    let nation = p.nation();
    if nation != 0 && p.outlet_type != -1 && p.pundit != -1 && p.home {
        covered!("nma_4 praise pundit");
        let g: unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8) = core::mem::transmute(0x7dad20usize);
        g(read_usize(G_PUNDIT_LIST), p.outlet_type as u32, p.pundit as u32, out, &mut p.outlet_gender);
        return;
    }
    covered!("nma_4 praise outlet type");
    // by type: (home text, form, abroad text)
    let (home, form, abroad) = match p.outlet_type.wrapping_sub(1) {
        0 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        1 => (0xa24284, 0x17, 0xa24250), // local newspaper
        2 => (0xa24238, 0x18, 0xa24208), // national radio station
        3 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        4 => (0xa241b0, 0x19, 0xa24184), // magazine
        5 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if p.home {
        f(out, home);
    } else {
        exe::nation_get_nationality_adjective(nation, p.adj(), 0x78, form);
        f(out, abroad, 0x64, p.adj());
    }
    p.outlet_gender = if p.outlet_type.wrapping_sub(1) == 4 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
}

/// The great player's name and the names of the player and the manager.
unsafe fn praise_names(p: &mut Praise) {
    let ids = |b: usize, n: [u32; 3], size: u32, style: u32, who: i32| {
        exe::database_format_name_from_ids(n[0], n[1], n[2], b as *mut u8, size, style, who as u32);
    };
    if p.legend > -1 {
        ids(p.lname_buf(), p.legend_names, 0x78, 1, p.legend);
    } else {
        let k = praise_rem(p.variant.wrapping_add(p.player), 4);
        let name = praise_frame::LEGENDS[if (0..3).contains(&k) { k as usize } else { 3 }];
        let n = (0..).take_while(|&i| read_u8(name + i) != 0).count() + 1;
        core::ptr::copy_nonoverlapping(name as *const u8, p.lname_buf() as *mut u8, n); // strcpy
    }
    ids(p.full(), p.names, 0x168, 1, p.player);
    ids(p.short(), p.names, 0x168, 5, p.player);
    ids(p.at(praise_frame::MANAGER_FULL), p.manager_names, 0x168, 1, p.manager);
    ids(p.at(praise_frame::MANAGER_SHORT), p.manager_names, 0x168, 5, p.manager);
}

/// The first paragraph of the body, by kind.
unsafe fn praise_body(p: &mut Praise) {
    match p.kind.wrapping_sub(0x29) {
        0 => praise_body_future_international(p),
        1 => praise_body_youngster(p),
        2 => praise_body_praised(p),
        4 => praise_body_on_the_rise(p),
        5 => praise_body_big_move(p),
        6 => praise_body_attitude(p),
        7 => praise_body_world_class(p),
        8 => praise_body_world_class_b(p),
        9 => praise_body_impresses(p),
        10 => praise_body_world_stage(p),
        11 => praise_body_competition(p),
        _ => praise_body_call_up(p),
    }
}

/// Kind 0x2e, "Big move close for {player}?": by variant, a pundit praising the club's player, the press speculating
/// about a move to a bigger club, a national newspaper calling him a big fish in a small pond, a club touting him, or a
/// television pundit heaping praise ("{nationality} ..." abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x745855..0x745ba9).
unsafe fn praise_body_big_move(p: &mut Praise) {
    covered!("nma_4 praise big move");
    p.piece = "body_big_move";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    match p.variant {
        0 => {
            f(h, 0xa433f8, cg, cg, p.pg(), p.pg(), cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa432e4, 0x64, cg, cg, g, g, full, cname, pos);
            } else {
                p.describe_keep();
                p.adjective(0x1c);
                f(h, 0xa431b0, 0x64, 0x64, 0x64, cg, cg, p.pg(), p.pg(), full, adj, cname, pos);
            }
        }
        2 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa430a4, g, g, cg, cg, pos, full, cname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa42f78, 0x64, g, g, cg, cg, adj, pos, full, cname);
            }
        }
        3 => {
            let g = p.describe();
            f(h, 0xa42e7c, cg, cg, g, g, cname, full, pos);
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa42d4c, cg, cg, 0x64, 0x64, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1f);
                let g = p.describe();
                f(h, 0xa42c00, 0x64, cg, cg, 0x64, g, g, adj, cname, full, short, pos);
            }
        }
    }
}

/// The great player's lines of kinds 0x30 and 0x31: with his club, (club gender, job gender, his club's gender,
/// his club's name, his job, his name); returns None without a club.
unsafe fn praise_legend_club(p: &mut Praise) -> Option<(i32, usize, usize)> {
    if p.legend_club <= -1 {
        return None;
    }
    let job = p.legend_job_name();
    let lc = p.legend_club_rec();
    Some((read_i8(lc + club::SHORT_GENDER_NAME) as i32, lc + club::SHORT_NAME, job))
}

/// Kind 0x30, "{player} world class talent": by variant, a former great (with his club and job, or his nation) or a
/// world cup winning manager saying the player will be a world star; variant 1 the same about his form this season;
/// the others, a household name throughout the world.
/// Original: part of `Media::write_player_praise_news_text` (0x745ba9..0x745ed9).
unsafe fn praise_body_world_class(p: &mut Praise) {
    covered!("nma_4 praise world class");
    p.piece = "body_world_class";
    let f = Praise::format();
    let (h, pos, full, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa426ac, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 3) != 0 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42598, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa424b4, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa429cc, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 13) > 7 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa428a8, cg, cg, pg, pg, 0x64, 0x64, ng, ng, 0x64, 0x64, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa427c4, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa42af4, cg, cg, g, g, cname, full, pos);
        }
    }
}

/// Kind 0x31, "{player} world class talent" (the second set): by variant, a former great or a world cup winning
/// manager saying the player is world class (with % 11 < 6 for the great's nation), or the player named among the
/// best in a world poll.
/// Original: part of `Media::write_player_praise_news_text` (0x745ed9..0x7461de).
unsafe fn praise_body_world_class_b(p: &mut Praise) {
    covered!("nma_4 praise world class b");
    p.piece = "body_world_class_b";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa41f78, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa41e78, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa41d94, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa42268, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42158, cg, cg, pg, pg, 0x64, ng, ng, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa42084, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            f(h, 0xa42384, cg, cg, 0x64, cname, full, short);
        }
    }
}

/// Kind 0x2d, "{player} on the rise": a media pundit's belief (variant 0), a television pundit's (variant 1), or a
/// national paper's report on his strengths (the others), "{nationality} ..." abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x7461de..0x746414).
unsafe fn praise_body_on_the_rise(p: &mut Praise) {
    covered!("nma_4 praise on the rise");
    p.piece = "body_on_the_rise";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let (lname, lg) = (p.lname(), p.lg());
    match p.variant {
        0 => {
            let g = p.describe();
            f(h, 0xa417cc, cg, cg, g, g, cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41c6c, cg, cg, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1c);
                let g = p.describe();
                f(h, 0xa41b28, cg, cg, g, g, 0x64, 0x64, cname, full, short, pos, adj);
            }
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41a04, g, g, 0x64, lg, lg, pos, full, lname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa418c0, 0x64, g, g, 0x64, lg, lg, adj, pos, full, lname);
            }
        }
    }
}

/// The nation of slot 7 (nation table + index × `nation::SIZE`), its name gender byte and name.
unsafe fn praise_nation_a(p: &Praise) -> (usize, i32) {
    let n = read_usize(G_NATION_TABLE).wrapping_add((p.nation_a as usize).wrapping_mul(nation::SIZE));
    (n + 4, read_i8(n + nation::GENDER_NAME) as i32)
}

/// Kind 0x29, "{player} rated highly / a 'future international' / set for call-up?": by variant, an outlet praising
/// him for the national team (variant 0), the club's player of the nation (1), a former great of that nation joining
/// the experts who want him called up (2), or a first cap in the offing (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746414..0x746851).
unsafe fn praise_body_future_international(p: &mut Praise) {
    covered!("nma_4 praise future international");
    p.piece = "body_future_international";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa40f08, og, og, cg, cg, g, g, 0x64, nag, nag, outlet, cname, pos, full, na);
            } else {
                f(h, 0xa40df0, og, og, cg, cg, g, g, 0x64, outlet, cname, pos, full);
            }
        }
        1 => {
            let pg = p.pg();
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa4115c, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
            } else {
                p.adjective(0x1c);
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa41044, cg, cg, pg, pg, 0x64, 0x64, nag, nag, cname, pos, full, adj, na);
            }
        }
        2 => {
            let pg = p.pg();
            if p.legend > -1 && read_i32(p.legend_nation()) == p.nation_a {
                if p.legend_club > -1 {
                    let job = p.legend_job_name();
                    let (na, nag) = praise_nation_a(p);
                    let lc = p.legend_club_rec();
                    let lcg = read_i8(lc + club::SHORT_GENDER_NAME) as i32;
                    let jg = p.jg();
                    f(h, 0xa41444, lcg, lcg, jg, jg, cg, cg, pg, pg, nag, nag, lc + club::SHORT_NAME, job, legend, cname, pos, full, na);
                } else {
                    let (na, nag) = praise_nation_a(p);
                    f(h, 0xa41334, cg, cg, pg, pg, nag, nag, legend, cname, pos, full, na);
                }
                return;
            }
            let (na, nag) = praise_nation_a(p);
            f(h, 0xa41254, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa416b0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa4157c, cg, cg, g, g, ng, ng, cname, pos, full, n + nation::SHORT_NAME);
            }
        }
    }
}

/// Kind 0x2a, "{player} shows promise / talented {player} given boost / youngster {player} praised": by variant, an
/// outlet praising the youngster (0), the club's young player (1, "{nationality}" abroad), an exciting future (2), or a
/// player type with a future (the others), with the club's nation abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x746851..0x746ac4).
unsafe fn praise_body_youngster(p: &mut Praise) {
    covered!("nma_4 praise youngster");
    p.piece = "body_youngster";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let og = p.outlet_gender as i8 as i32;
            f(h, 0xa406d0, og, og, cg, cg, p.pg(), p.pg(), 0x64, outlet, cname, pos, full);
        }
        1 => {
            if p.home {
                f(h, 0xa408c0, cg, cg, p.pg(), p.pg(), cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa407e4, cg, cg, p.pg(), p.pg(), 0x64, 0x64, cname, pos, full, adj);
            }
        }
        2 => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40aa0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40978, cg, cg, g, g, ng, ng, cname, pos, full, n + nation::SHORT_NAME);
            }
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40cdc, g, g, cg, cg, pos, full, cname);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40bb0, g, g, ng, ng, cg, cg, pos, full, n + nation::SHORT_NAME, cname);
            }
        }
    }
}

/// Kind 0x32, "{player} impresses at {club}": an outlet praising his impact (variant 0), an outlet on his type
/// (1), or his performances since joining (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746ac4..0x746bd1).
unsafe fn praise_body_impresses(p: &mut Praise) {
    covered!("nma_4 praise impresses");
    p.piece = "body_impresses";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    match p.variant {
        0 => {
            f(h, 0xa40608, og, og, cg, cg, outlet, cname, full);
        }
        1 => {
            let g = p.describe();
            f(h, 0xa40530, og, og, g, g, cg, cg, outlet, pos, full, cname);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa403ec, cg, cg, g, g, full, cname, pos, short);
        }
    }
}

/// strcat(dst, src).
unsafe fn praise_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// strcmp(a, b) == 0.
unsafe fn praise_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// The match of kind 0x34: the record's club (+0x15), its long-name gender and name, the stage gender (frame +0xe),
/// the score text (frame +0x4c) and the stage label (EXTRA).
struct PraiseMatch {
    rg: i32,
    rname: usize,
    sg: i32,
    score: usize,
    stage: usize,
}

/// The rival club (slot "rival club") of kind 0x34: its short-name gender and name, its nation's gender and name.
struct PraiseRival {
    rgs: i32,
    name: usize,
    rng: i32,
    nation_name: usize,
}

unsafe fn praise_rival(p: &Praise) -> PraiseRival {
    let r = read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE));
    let n = read_usize(r + club::NATION);
    PraiseRival {
        rgs: read_i8(r + club::SHORT_GENDER_NAME) as i32,
        name: r + club::SHORT_NAME,
        rng: read_i8(n + nation::SHORT_GENDER_NAME) as i32,
        nation_name: n + nation::SHORT_NAME,
    }
}

/// The date of values 2 and 3 as a long date without the year (`Date::get_long_date_string_no_year`, 0x78, no time of
/// day) into the frame at +0xc58.
unsafe fn praise_date_text(p: &Praise) -> usize {
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, p.m2 as u32, p.m3 as u32);
    let g: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54bef0usize);
    let out = p.at(praise_frame::DATE_TEXT);
    g(date.as_mut_ptr() as usize, out, 0x78, 0xff);
    out
}

/// Kind 0x34, "{player} lights up / stars in {competition}": the player's part in a match of a competition (the match
/// record of slot 17): the score (higher first, "%d-%d") and the stage label (`comp_util_format_comp_stage_round_label`,
/// "Friendly" and "Reserve" with a small first letter; gender `comp_util_get_label_gender`). By variant and whether the
/// club won (record +0x19 above +0x1a): a paragraph on the match, then, with a rival club, a second one (the player due
/// to sign for it on a date when value 1 is set; attracting interest while playing in a nation when value 4 is set;
/// playing for it otherwise); after a draw or a defeat, value 11 adds how it ended (won or lost on penalties or a
/// shootout). Without the match record: the original's error box (media.cpp line 0x4743) and an empty paragraph.
/// Original: part of `Media::write_player_praise_news_text` (0x746bd1..0x7483d2).
unsafe fn praise_body_competition(p: &mut Praise) {
    covered!("nma_4 praise competition");
    p.piece = "body_competition";
    let f = Praise::format();
    let h = p.head();
    if p.has_match == 0 {
        covered!("nma_4 praise competition without match");
        f(h, 0xad9c64); // ""
        praise_error_box(p, 0x4743);
        return;
    }
    p.describe_keep();
    let rec = p.record.as_ptr() as usize;
    let (a, b) = (read_i8(rec + 0x19), read_i8(rec + 0x1a));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let score = p.at(0x4c);
    sprintf(score, 0x98c114, a.max(b) as i32, a.min(b) as i32); // "%d-%d"
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(rec + 8) as usize).wrapping_mul(0x6b));
    let stage = p.at(praise_frame::EXTRA);
    exe::comp_util_format_comp_stage_round_label(comp, read_u32(rec + 0xc), read_u32(rec + 0xe), 1, stage as *mut u8, 0xc8);
    if praise_same_text(stage, 0x9db958) {
        write_u8(stage, b'f'); // "Friendly"
    } else if praise_same_text(stage, 0x9b5b40) {
        write_u8(stage, b'r'); // "Reserve"
    }
    p.job_gender = exe::comp_util_get_label_gender(comp as u32, read_u32(rec + 0xc), read_u32(rec + 0xe), 1);
    let rc = read_usize(G_CLUB_TABLE).wrapping_add((read_u32(rec + 0x15) as usize).wrapping_mul(club::SIZE));
    let m = PraiseMatch { rg: read_i8(rc + club::GENDER_NAME) as i32, rname: rc + club::NAME, sg: p.jg(), score, stage };
    let won = a > b;
    let ended = match p.variant {
        0 => praise_competition_v0(p, &m, won),
        1 => praise_competition_v1(p, &m, won),
        2 => praise_competition_v2(p, &m, won),
        _ => praise_competition_other(p, &m, won),
    };
    if ended {
        return;
    }
    // after a draw or a defeat: how it ended
    let tail = match p.m11 {
        1 => 0xa21e0c, // won after a penalty shootout
        2 => 0xa21684, // lost on penalties
        3 => 0xa21dc0, // won after a shootout
        4 => 0xa21d74, // lost on a shootout
        _ => return,
    };
    covered!("nma_4 praise competition shootout");
    let (cg, cname, r) = (p.cg(), p.cname(), p.reply());
    f(r, tail, cg, cg, cname);
    praise_strcat(h, r);
}

/// The original's assertion box: the file and line (`crt` splitpath of the media.cpp path), the version, "Error"
/// (`gui_show_message_box`), and the mouse button released.
unsafe fn praise_error_box(p: &Praise, line: u32) {
    let (name, ext) = (p.at(praise_frame::SPLIT_NAME), p.at(praise_frame::SPLIT_EXT));
    exe::crt_strtok(0xa17bec, 0, 0, name as *mut u8, ext as *mut u8);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(0xae24d0, 0x9870e8, exe::utils_get_version_string(), name, ext, line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Variant 0 of kind 0x34 (the player shone in the match). Returns true when the paragraph is complete (a win).
unsafe fn praise_competition_v0(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg) = (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        let (first, second) = match (won, p.m1 != 0, p.m4 != 0) {
            (true, true, _) => (0xa3eb20, 0xa3ea64),
            (true, false, true) => (0xa3eb20, 0xa3e980),
            (true, false, false) => (0xa3eb20, 0xa3e8c8),
            (false, true, _) => (0xa3e6b4, 0xa3e5f4),
            (false, false, true) => (0xa3e6b4, 0xa3e54c),
            (false, false, false) => (0xa3e6b4, 0xa3e8c8),
        };
        f(h, first, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
        if second == 0xa3e54c {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, rv.name);
        } else {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, short, rv.name);
        }
        praise_strcat(h, r);
    } else {
        let t = if won { 0xa3e77c } else { 0xa3e400 };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, full, cname, m.score, m.stage, m.rname, pos, short);
    }
    won
}

/// Variant 1 of kind 0x34 (the player starred in the match). Returns true after a win.
unsafe fn praise_competition_v1(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, pg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3f420, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, 0x64, full, cname, m.score, m.stage, m.rname, rv.name, short);
        } else if !won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3ee64, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, pg, pg, full, cname, m.score, m.stage, m.rname, rv.name, pos);
        } else {
            let t = match (won, p.m1 != 0) {
                (true, true) => 0xa3f5f8,
                (true, false) => 0xa3f2d0,
                (false, true) => 0xa3f050,
                (false, false) => 0xa3ed14,
            };
            f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, full, cname, m.score, m.stage, m.rname, rv.name);
        }
    } else {
        let t = if won { 0xa3f1a4 } else { 0xa3ebe4 };
        f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
    }
    won
}

/// Variant 2 of kind 0x34 (after helping the club to the score). Returns true after a win.
unsafe fn praise_competition_v2(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, full, short, cname, cg) = (p.head(), p.full(), p.short(), p.cname(), p.cg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if p.m1 != 0 {
            let t = if won { 0xa3fd7c } else { 0xa3f9a8 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, full, short, rv.name);
        } else {
            let t = if won { 0xa3fc3c } else { 0xa3f868 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, rv.name, full);
        }
    } else {
        let t = if won { 0xa3fb20 } else { 0xa3f74c };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, cname, m.score, m.stage, m.rname, full);
    }
    won
}

/// The other variants of kind 0x34 (after the club saw off the challenge / played out the score). Returns true after
/// a win.
unsafe fn praise_competition_other(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg, legend) =
        (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg(), p.lname_buf());
    let first = |h: usize| {
        if won {
            f(h, 0xa40290, cg, cg, m.rg, m.rg, m.sg, m.sg, pg, pg, 0x64, 0x64, cname, m.rname, m.score, m.stage, pos, full, legend);
        } else {
            f(h, 0xa3ff94, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, 0x64, 0x64, cname, m.score, m.stage, m.rname, pos, full, legend);
        }
    };
    if p.rival_club <= -1 {
        first(h);
        return won;
    }
    let rv = praise_rival(p);
    if p.m1 != 0 {
        let date = praise_date_text(p);
        first(h);
        f(r, 0xa40214, 0x64, rv.rgs, rv.rgs, 0x64, short, rv.name, date);
    } else {
        first(h);
        if p.m4 != 0 && won {
            f(r, 0xa40168, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else if p.m4 != 0 {
            f(r, 0xa3fef0, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else {
            f(r, 0xa400e8, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        }
    }
    praise_strcat(h, r);
    won
}

/// A nation by index (nation table + index × `nation::SIZE`, unchecked as in the original): its short-name gender
/// byte and short name.
unsafe fn praise_nation_by_index(i: i32) -> (i32, usize) {
    let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE));
    (read_i8(n + nation::SHORT_GENDER_NAME) as i32, n + nation::SHORT_NAME)
}

/// The competition of slot 16 (club comp table + index × 0x6b): its short-name gender byte and short name.
unsafe fn praise_comp(p: &Praise) -> (i32, usize) {
    let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
    (read_i8(c + competition::SHORT_GENDER_NAME) as i32, c + competition::SHORT_NAME)
}

/// Kind 0x33, "{player} tipped to shine on world stage": the player in a competition (slot 16) with two nations
/// (values 9 and 10): by variant, an outlet praising his performances (0), a former international star or a former
/// great with his club and job predicting he will shine (1 and 2; variant 2 names the great's nation when he has no
/// club, and a great's tip without one), or "{nationality} player type" (the others, adjective form 0xc). The nations
/// are left out when one of them is the club's nation, and the second one when either is missing. With a rival club,
/// the paragraph is then kept as the first sentence of one about his form for that club (when value 1 is 0 and the
/// great % 3 is not 0) or about signing for it on a date (value 1 set).
/// Original: part of `Media::write_player_praise_news_text` (0x7483d2..0x74948e).
unsafe fn praise_body_world_stage(p: &mut Praise) {
    covered!("nma_4 praise world stage");
    p.piece = "body_world_stage";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    let (compg, compname) = praise_comp(p);
    let cid = read_i32(p.nation());
    let own = p.nation_b == cid || p.nation_c == cid;
    let both = p.nation_b > -1 && p.nation_c > -1;
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if own {
                if p.home {
                    f(h, 0xa3c97c, og, og, g, g, compg, compg, outlet, pos, full, compname);
                } else {
                    f(h, 0xa3ca9c, og, og, cg, cg, g, g, 0x64, compg, compg, outlet, cname, pos, full, compname);
                }
            } else if both {
                if p.home {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3ce7c, og, og, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, pos, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3cfdc, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, cname, pos, full, compname, nbname, ncname);
                }
            } else if p.home {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cbe4, og, og, g, g, 0x64, compg, compg, nbg, nbg, outlet, pos, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cd1c, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, outlet, cname, pos, full, compname, nbname);
            }
        }
        1 => {
            p.describe();
            if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d25c, lcg, lcg, jg, jg, cg, cg, 0x64, compg, compg, lcname, job, legend, cname, full, compname);
                } else {
                    f(h, 0xa3d164, cg, cg, 0x64, compg, compg, legend, cname, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d6d8, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d5a4, cg, cg, 0x64, compg, compg, nbg, nbg, ncg, ncg, legend, cname, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d47c, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, lcname, job, legend, cname, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d370, cg, cg, compg, compg, nbg, nbg, legend, cname, full, compname, nbname);
            }
        }
        2 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend == -1 {
                f(h, 0xa3e034, cg, cg, pg, pg, compg, compg, legend, cname, pos, full, compname);
            } else if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d970, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, lcname, job, legend, cname, pos, full, compname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa3d828, lng, lng, cg, cg, pg, pg, 0x64, compg, compg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dec0, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, pos, full, compname, nbname, ncname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dd50, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dc04, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, lcname, job, legend, cname, pos, full, compname, nbname);
            } else {
                let ln = p.legend_nation();
                let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dabc, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname);
            }
        }
        _ => {
            p.adjective(0xc);
            let pg = p.pg();
            if own {
                f(h, 0xa3e120, 0x64, 0x64, pg, pg, compg, compg, adj, pos, full, compname);
            } else if both {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                f(h, 0xa3e2e8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, ncg, ncg, adj, pos, full, compname, nbname, ncname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3e1f8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, adj, pos, full, compname, nbname);
            }
        }
    }
    if p.rival_club <= -1 {
        return;
    }
    let r = p.reply();
    let rc = read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE));
    let rlg = read_i8(rc + club::GENDER_NAME) as i32;
    if p.m1 == 0 {
        if praise_rem(p.legend, 3) == 0 {
            return;
        }
        covered!("nma_4 praise world stage form");
        praise_strcpy(r, h);
        f(h, 0xa3c8f0, rlg, rlg, r, p.short(), rc + club::NAME);
    } else {
        covered!("nma_4 praise world stage signing");
        let date = praise_date_text(p);
        praise_strcpy(r, h);
        f(h, 0xa3c860, rlg, rlg, 0x64, r, p.short(), rc + club::NAME, date);
    }
}

/// strcpy(dst, src).
unsafe fn praise_strcpy(dst: usize, src: usize) {
    write_u8(dst, 0);
    praise_strcat(dst, src);
}

/// Kind 0x2b, "{player} praised": by variant 0..3, an outlet praising his form (strength 0x1000000) or journalists
/// hailing him ("in {nation}" abroad), a club's player of the nation, an outlet lost for words ("{nationality} pundit"
/// abroad), an outlet lavishing praise. Other variants go by the strength flags when variant % 4 is not 0: 0x2c0040
/// lethal striker, 0x240040 striker of the competition (long name), 0xc0040 hotshot, 0x40040 striker of the
/// competition, 0x102040, 0x200840 and 0x840 strikers; otherwise the description is kept and, with strength
/// 0x1000000, the media praise him ("{nationality} media" abroad), else the variant becomes club % 4 (C remainder) and
/// the choice starts again (a negative club index with none of these flags loops for ever, as in the original).
/// Original: part of `Media::write_player_praise_news_text` (0x74948e..0x749b63).
unsafe fn praise_body_praised(p: &mut Praise) {
    covered!("nma_4 praise praised");
    p.piece = "body_praised";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    let s = p.strengths;
    let nation = |p: &Praise| {
        let n = p.nation();
        (read_i8(n + nation::SHORT_GENDER_NAME) as i32, n + nation::SHORT_NAME)
    };
    let comp_long = |p: &Praise| {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        (read_i8(c + competition::GENDER_NAME) as i32, c + competition::NAME)
    };
    let mut v = p.variant;
    loop {
        match v {
            0 => {
                if s & 0x100_0000 != 0 {
                    f(h, 0xa3c754, og, og, cg, cg, 0x64, 0x64, outlet, cname, full, short);
                } else if p.home {
                    f(h, 0xa3c654, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c538, ng, ng, cg, cg, p.pg(), p.pg(), 0x64, nname, cname, pos, full);
                }
                return;
            }
            1 => {
                if p.home {
                    f(h, 0xa3c480, cg, cg, cname, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c3c8, cg, cg, ng, ng, cname, full, nname);
                }
                return;
            }
            2 => {
                if p.home {
                    f(h, 0xa3c300, og, og, cg, cg, outlet, cname, full);
                } else {
                    p.adjective(0x1f);
                    f(h, 0xa3c240, 0x64, cg, cg, adj, cname, full);
                }
                return;
            }
            3 => {
                f(h, 0xa3c164, og, og, cg, cg, outlet, cname, full);
                return;
            }
            _ => {}
        }
        let odd = praise_rem(v, 4) != 0;
        if s & 0x2c0040 == 0x2c0040 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3c030, cg, cg, 0x64, ng, ng, cname, full, short, nname);
            return;
        }
        if s & 0x240040 == 0x240040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bee0, cg, cg, 0x64, clg, clg, cname, full, short, clname);
            return;
        }
        if s & 0xc0040 == 0xc0040 && odd {
            if p.home {
                f(h, 0xa3bdf4, cg, cg, cname, full);
            } else {
                p.adjective(0x16);
                f(h, 0xa3bcfc, cg, cg, 0x64, cname, full, adj);
            }
            return;
        }
        if s & 0x40040 == 0x40040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bc10, cg, cg, clg, clg, cname, full, clname);
            return;
        }
        if s & 0x102040 == 0x102040 && odd {
            if p.home {
                f(h, 0xa3bb60, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3ba90, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x200840 == 0x200840 && odd {
            if p.home {
                f(h, 0xa3b9e0, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3b910, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x840 == 0x840 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3b7d0, cg, cg, ng, ng, cname, full, short, nname);
            return;
        }
        p.describe_keep();
        if s & 0x100_0000 != 0 {
            if p.home {
                f(h, 0xa3b6cc, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa3b5a8, 0x64, 0x64, cg, cg, p.pg(), p.pg(), 0x64, adj, cname, pos, full);
            }
            return;
        }
        v = praise_rem(p.club_index, 4);
    }
}

/// Kind 0x2f, "{player} attitude praised": a media pundit's belief (variant 0) or a national newspaper columnist's.
/// Original: part of `Media::write_player_praise_news_text` (0x749b63..0x749c05).
unsafe fn praise_body_attitude(p: &mut Praise) {
    covered!("nma_4 praise attitude");
    p.piece = "body_attitude";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    if p.variant != 0 {
        f(h, 0xa3b48c, cg, cg, g, g, full, cname, pos);
    } else {
        f(h, 0xa3b378, g, g, cg, cg, pos, full, cname);
    }
}

/// Kind 0x2c (and any kind out of 0x29..0x34), "{player} set for call-up?": national papers lavishing praise, with
/// the rumour of a call-up to the nation of slot 7 ("in {nation}" abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x749c05..0x749d35).
unsafe fn praise_body_call_up(p: &mut Praise) {
    covered!("nma_4 praise call up");
    p.piece = "body_call_up";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    let (na, nag) = praise_nation_a(p);
    if p.home {
        f(h, 0xa3b258, cg, cg, g, g, nag, nag, cname, full, pos, na);
    } else {
        let n = p.nation();
        let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
        f(h, 0xa3b118, ng, ng, cg, cg, g, g, nag, nag, n + nation::SHORT_NAME, cname, full, pos, na);
    }
}

/// The second paragraph: the reply of the manager (slot 13: 1, 2, 3 for none, other a blank), in the second person
/// when the manager is the human on screen, by kind (and, for the players' age [inference], by value 16 against 24
/// and 34, or 36 with value 17).
/// Original: part of `Media::write_player_praise_news_text` (0x749d35..0x74a30c).
unsafe fn praise_reply(p: &mut Praise) {
    let f = Praise::format();
    let (r, short) = (p.reply(), p.short());
    let manager = p.at(praise_frame::MANAGER_FULL);
    let k = p.kind.wrapping_sub(0x29);
    let k = if (0..12).contains(&k) { k } else { -1 };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let older = if p.m17 != 0 { p.age > 0x24 } else { p.age > 0x22 };
    let blank = || {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(r, 0x9f1308); // " "
    };
    if human == staff_of(p.manager) {
        covered!("nma_4 praise reply human");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa3aaa0);
                }
                1 => {
                    f(r, 0xa3aa38, short);
                }
                3 => {
                    f(r, 0xa3a9d8);
                }
                4 | 7 => {
                    f(r, 0xa3a978, short);
                }
                5 => {
                    f(r, 0xa3aafc);
                }
                6 => {
                    f(r, 0xa3ab4c);
                }
                8 => {
                    f(r, 0xa3a908, short);
                }
                9 => {
                    f(r, 0xa3a8a0, 0x64, short);
                }
                10 => {
                    f(r, 0xa3a7c8, 0x64, short);
                }
                11 => {
                    f(r, 0xa3a83c, 0x64, short);
                }
                _ => {
                    f(r, 0xa3a754, 0x64, short);
                }
            },
            2 => match k {
                0 | 1 | 4 | 5 | 7 => {
                    f(r, 0xa3b008, 0x64, short);
                }
                3 => {
                    f(r, 0xa3af90, short);
                }
                6 => {
                    f(r, 0xa3b05c, 0x64, short);
                }
                8 => {
                    f(r, 0xa3af08, short);
                }
                9 => {
                    f(r, 0xa3ae80, short);
                }
                10 => {
                    f(r, 0xa3ad8c, short);
                }
                11 => {
                    f(r, 0xa3adf8, short);
                }
                _ => {
                    let t = if older {
                        0xa3ac40
                    } else if p.age >= 0x18 {
                        0xa3aba4
                    } else {
                        0xa3ad04
                    };
                    f(r, t, short);
                }
            },
            3 => {
                f(r, 0xa3b0e0); // "You have chosen not to reply to this press statement."
            }
            _ => blank(),
        }
    } else {
        covered!("nma_4 praise reply manager");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa39f14, manager);
                }
                1 => {
                    f(r, 0xa39e84, manager, short);
                }
                3 => {
                    f(r, 0xa39e00, manager);
                }
                4 | 7 => {
                    f(r, 0xa39d80, manager, short);
                }
                5 => {
                    f(r, 0xa39f98, manager);
                }
                6 => {
                    f(r, 0xa3a010, manager);
                }
                8 => {
                    f(r, 0xa39d0c, manager, short);
                }
                9 => {
                    f(r, 0xa39c80, 0x64, manager, short);
                }
                10 => {
                    f(r, 0xa39b5c, 0x64, manager, short);
                }
                11 => {
                    f(r, 0xa39bf8, 0x64, manager, short);
                }
                _ => {
                    f(r, 0xa39ac4, 0x64, manager, short);
                }
            },
            2 => {
                let t = match k {
                    0 | 1 | 4 | 5 | 7 => 0xa3a630,
                    3 => 0xa3a588,
                    6 => 0xa3a6ac,
                    8 => 0xa3a4d8,
                    9 => 0xa3a430,
                    10 => 0xa3a2ec,
                    11 => 0xa3a384,
                    _ => {
                        if older {
                            0xa3a14c
                        } else if p.age >= 0x18 {
                            0xa3a08c
                        } else {
                            0xa3a238
                        }
                    }
                };
                f(r, t, 0x64, 0x64, manager, short);
            }
            3 => {
                f(r, 0x9f1308); // " "
            }
            _ => blank(),
        }
    }
}

/// The headline: "Praise for {club} {position}" when variant % 8 is player % 7 (C remainders), otherwise by kind:
/// rated highly, a 'future international' or set for call-up (0x29, by (variant + player) % 3), shows promise, given
/// boost or youngster praised (0x2a, the same), praised, set for call-up, on the rise, big move close, attitude
/// praised, world class talent (0x30, 0x31), impresses at {club}, tipped to shine on world stage, lights up or stars in
/// {competition} (0x34, by (variant × player) % 17 above 6), touted (other kinds). The match link's text (frame +0x4c)
/// is emptied.
/// Original: part of `Media::write_player_praise_news_text` (0x74a33f..0x74a5d4).
unsafe fn praise_headline(p: &mut Praise) {
    covered!("nma_4 praise headline");
    p.piece = "headline";
    let f = Praise::format();
    let (h, short) = (p.head(), p.short());
    write_u8(p.at(0x4c), 0);
    if praise_rem(p.variant, 8) == praise_rem(p.player, 7) {
        f(h, 0xa39a70, p.cg(), p.cg(), p.pg(), p.pg(), p.cname(), p.pos());
        return;
    }
    let three = praise_rem(p.variant.wrapping_add(p.player), 3);
    match p.kind.wrapping_sub(0x29) {
        0 => {
            let t = match three {
                0 => 0xa39900,
                1 => 0xa39930,
                _ => 0xa3996c,
            };
            f(h, t, short);
        }
        1 => {
            let t = match three {
                0 => 0xa39864,
                1 => 0xa39894,
                _ => 0xa398cc,
            };
            f(h, t, short);
        }
        2 => {
            f(h, 0xa39708, short);
        }
        3 => {
            f(h, 0xa3996c, short);
        }
        4 => {
            f(h, 0xa399a0, short);
        }
        5 => {
            f(h, 0xa39a04, short);
        }
        6 => {
            f(h, 0xa39a3c, 0x64, short);
        }
        7 | 8 => {
            f(h, 0xa399d0, short);
        }
        9 => {
            f(h, 0xa39810, p.cg(), p.cg(), short, p.cname());
        }
        10 => {
            f(h, 0xa39734, short);
        }
        11 => {
            let (compg, compname) = praise_comp(p);
            let t = if praise_rem(p.variant.wrapping_mul(p.player), 0x11) > 6 { 0xa397c0 } else { 0xa39774 };
            f(h, t, compg, compg, short, compname);
        }
        _ => {
            f(h, 0xa396e0, short);
        }
    }
}

/// The links of the text (see the routine's header).
unsafe fn praise_links(p: &mut Praise, out: usize) {
    covered!("nma_4 praise links");
    let link = |name: usize, kind: u32, data: usize| {
        exe::text_add_link(out, name as *const u8, kind, data);
    };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let club_of = |i: i32| read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE));
    let national = |n: usize| exe::database_get_national_team(n);
    let rec = p.record.as_ptr() as usize;
    if p.has_match != 0 {
        // Match_Stats::fixture_entry_matches(index, club a, club b, day, year) (thiscall, ret 0x14; the dwords pushed)
        let g: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(0x729500usize);
        if g(read_usize(G_MATCH_STATS), read_i32(rec + 0x1b), read_i32(rec + 0x11), read_i32(rec + 0x15), read_i32(rec), read_i32(rec + 2)) != 0 {
            link(p.at(0x4c), 0x7d3, read_usize(rec + 0x1b));
        }
        let c = club_of(read_i32(rec + 0x15));
        link(c + club::NAME, 0x7d5, c);
    }
    if p.player > -1 && p.player < read_i32(G_STAFF_COUNT)
        && exe::database_staff_matches_names(staff_of(p.player), p.names[0], p.names[1], p.names[2], 0usize) != 0
    {
        link(p.full(), 0x7d7, staff_of(p.player));
        link(p.short(), 0x7d7, staff_of(p.player));
    }
    if p.manager > -1
        && exe::database_staff_matches_names(staff_of(p.manager), p.manager_names[0], p.manager_names[1], p.manager_names[2], 0usize) != 0
    {
        link(p.at(praise_frame::MANAGER_FULL), 0x7d7, staff_of(p.manager));
        link(p.at(praise_frame::MANAGER_SHORT), 0x7d7, staff_of(p.manager));
    }
    if p.legend > -1
        && exe::database_staff_matches_names(staff_of(p.legend), p.legend_names[0], p.legend_names[1], p.legend_names[2], 0usize) != 0
    {
        link(p.lname_buf(), 0x7d7, staff_of(p.legend));
    }
    for c in [p.legend_club, p.rival_club] {
        if c != -1 {
            let r = club_of(c);
            link(r + club::SHORT_NAME, 0x7d5, r);
            link(r + club::NAME, 0x7d5, r);
        }
    }
    if p.club_index != -1 {
        link(p.cname(), 0x7d5, p.club);
        link(p.lname(), 0x7d5, p.club);
        let n = p.nation();
        link(n + 4, 0x7d5, national(n));
        link(n + nation::SHORT_NAME, 0x7d5, national(n));
        for form in 0x16..=0x1f {
            exe::nation_get_nationality_adjective(n, p.adj(), 0x78, form);
            link(p.adj(), 0x7d5, national(n));
        }
    }
    for i in [p.nation_a, p.nation_b, p.nation_c] {
        if i != -1 {
            let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE));
            link(n + nation::SHORT_NAME, 0x7d5, national(n));
            link(n + 4, 0x7d5, national(n));
        }
    }
    if p.comp != -1 {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        link(c + competition::NAME, 0x7d0, c);
        link(c + competition::SHORT_NAME, 0x7d0, c);
    }
}

/// The bytes of a C string (empty for a null pointer).
unsafe fn nma4_c_text(p: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if p == 0 {
        return v;
    }
    let mut k = 0;
    while read_u8(p + k) != 0 {
        v.push(read_u8(p + k));
        k += 1;
    }
    v
}

/// The links of a Text object (`+4` the link list): the list's cursor (+0xa) starts at its head (+2) and each step
/// (as `0x68d1b0` does) takes the node's link (+0) and moves to the next (+6); a link holds its text at +0, its type at
/// +0xc8 and its data at +0xcc. The cursor is put back afterwards.
unsafe fn nma4_links(text: usize) -> Vec<(Vec<u8>, u32, u32)> {
    let mut v = Vec::new();
    let list = read_usize(text + 4);
    if list == 0 {
        return v;
    }
    let saved = read_u32(list + 0xa);
    write_u32(list + 0xa, read_u32(list + 2));
    loop {
        let node = read_usize(list + 0xa);
        if node == 0 {
            break;
        }
        let link = read_usize(node);
        write_u32(list + 0xa, read_u32(node + 6));
        v.push((nma4_c_text(link), read_u32(link + 0xc8), read_u32(link + 0xcc)));
    }
    write_u32(list + 0xa, saved);
    v
}

/// Comparer for `write_player_praise_news_text`: the original writes into `out`; the copy, side by side (the text
/// buffer 0xde1f64 and the generator's state), into a local empty Text; then the strings and the new links (text, type,
/// data) are compared and the local Text is destroyed. A divergence names the piece that wrote the text. The match
/// link's text (0x7d3) is not compared in the body of a kind other than 0x34 (the original's uninitialised frame byte).
pub(crate) unsafe extern "thiscall" fn compare_write_player_praise_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_PLAYER_PRAISE_NEWS_TEXT);
    let before = nma4_links(out).len();
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let est = crate::rng::STATE_REGIONS;
    let regions = [(PRAISE_TEXT_BUFFER, 2000usize), est[0], est[1]];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(this, news, body, out),
        || crate::rng::without_counting(|| write_player_praise_news_text(this, news, body, lp)));
    let piece = PRAISE_LAST_PIECE;
    let (so, sr) = (nma4_c_text(read_usize(out)), nma4_c_text(read_usize(lp)));
    let lo: Vec<_> = nma4_links(out).into_iter().skip(before).collect();
    let lr = nma4_links(lp);
    let kind = exe::newsitem_get_slot(news, 5);
    let loose = body != 0 && kind != 0x34;
    let links_same = lo.len() == lr.len()
        && lo.iter().zip(lr.iter()).all(|(a, b)| a.1 == b.1 && a.2 == b.2 && (a.0 == b.0 || (loose && a.1 == 0x7d3)));
    SCOREBOARD.record(same && so == sr && links_same, || format!(
        "write_player_praise_news_text(kind {:#x}, body {}, piece {}) text {} links {} ({} x {}) memory {}\n original: {}\n rust:     {}\n links original {:x?}\n links rust {:x?}\n {:x?} x {:x?}",
        kind, body, piece, so == sr, links_same, lo.len(), lr.len(), same, String::from_utf8_lossy(&so), String::from_utf8_lossy(&sr),
        lo.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
        lr.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(), mo, mr));
    exe::drop_text(local.as_mut_ptr());
}

// ---------- batch 294: the transfer rumour text and the team media story (media.cpp) ----------

/// The piece of `write_transfer_rumour_news_text` that ran last, for the comparer's message: the low byte is the lead
/// (0 the headline, 1 the lead of kind 0, 2 of kind 1, 3 of kind 36, 4 the others), the next byte the clubs paragraph
/// (the number of clubs, 2 to 5) and the third the agent line (1 the human manager is the agent, 2 the others).
pub(crate) static TRANSFER_RUMOUR_PIECE: AtomicU32 = AtomicU32::new(0);

/// The text buffer the media texts are formatted into before they go into the Text object.
const RUMOUR_TEXT: usize = 0xde1f64;

/// The club name (`club::NAME`) of club `id` of the club table.
unsafe fn rumour_club_name(id: i32) -> usize {
    club_at(id as isize).addr() + club::NAME
}

/// The grammatical gender byte of the club name (`club::GENDER_NAME`), sign-extended.
unsafe fn rumour_club_gender(id: i32) -> i32 {
    club_at(id as isize).gender_name() as i32
}

/// The short club name (`club::SHORT_NAME`).
unsafe fn rumour_club_short(id: i32) -> usize {
    club_at(id as isize).addr() + club::SHORT_NAME
}

/// The grammatical gender byte of the short club name (`club::SHORT_GENDER_NAME`), sign-extended.
unsafe fn rumour_club_short_gender(id: i32) -> i32 {
    club_at(id as isize).short_gender_name() as i8 as i32
}

/// Appends the text buffer to the Text object (`Text::append_string`, 0x4ae770, thiscall, ret 4).
unsafe fn rumour_append(out: usize, s: usize) {
    let append: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x4ae770usize);
    append(out, s);
}

/// The values the transfer rumour text reads from its news item, by the original's frame: `f` holds the words from
/// +0x00 to +0xb0 of the frame (index = offset / 4), where the club list (+0x28, one club per slot from 0x10) runs over
/// the slots kept after it when it has more than five clubs.
struct Rumour {
    f: [u32; 0x2c],
    club: i32,      // slot 4: the bidding club (-1 none)
    lead: i32,      // slot 5: the kind of lead sentence (0, 1, 36 or other)
    position: u32,  // slot 9: the player's position flags
    flag14: u8,     // slot 14: a flag of the agent line
    count: i32,     // slot 15: the number of clubs in the list (0 when the list reached slot 0x32)
    nation: usize,  // the nation chosen for the links and the nationality adjective (0 none)
    local: bool,    // the human manager is of that nation (or there is no nation)
    desc_gender: i32,
}

impl Rumour {
    fn player(&self) -> i32 { self.f[0x50 / 4] as i32 }      // slot 0: the player's id
    fn player_names(&self) -> (u32, u32, u32) { (self.f[0x4c / 4], self.f[0x44 / 4], self.f[0x40 / 4]) } // slots 1-3
    fn fee(&self) -> u32 { self.f[0x5c / 4] }                // slot 6
    fn agent_kind(&self) -> i32 { self.f[0x64 / 4] as i32 }  // slot 7
    fn qualities(&self) -> u32 { self.f[0x48 / 4] }          // slot 8: the player's strength flags
    fn agent(&self) -> i32 { self.f[0x3c / 4] as i32 }       // slot 10: the agent's id (-1 none)
    fn agent_names(&self) -> (u32, u32, u32) { (self.f[0x60 / 4], self.f[0x58 / 4], self.f[0x54 / 4]) } // slots 11-13
    fn listed(&self, i: usize) -> i32 { self.f[0x28 / 4 + i] as i32 }
}

/// `Media::write_transfer_rumour_news_text(news, body, out)`: writes the headline or the body of a transfer rumour
/// story into `out`, with the links.
///
/// It reads slots 0 to 14 of the news item (`exe::newsitem_get_slot`): the player (0) and his name ids (1-3), the
/// bidding club (4, −1 none), the kind of lead (5), the fee (6), the agent line kind (7), the player's strength (8)
/// and position (9) flags, the agent (10) and his name ids (11-13) and a flag (14); slot 15 is the number of clubs
/// in the story, read from slot 0x10 on (the list stops at slot 0x32 and then counts as empty; the original keeps it
/// in a 5-club frame array, and longer lists run over the slots kept after it, which the copy reproduces). The
/// player's position name (`write_position_name`) and description (`write_player_description`, with its gender).
/// The nation of the story, by (count + agent + lead + club + player) % 6 with a club: 1-2 the club's nation, 3 the
/// player's nation if its reputation is 6000 or more, 4-5 the first listed club's nation; otherwise none. The
/// "local" flag: the human manager of the current screen (`G_SCREEN_RECORDS_MANAGER`) with no nation, of a club
/// (`.club_job()`) of that nation or, without a club, of that nationality.
/// The headline (`transfer_rumour_headline`) or the body: the lead by kind (`transfer_rumour_lead_kind0`,
/// `_kind1`, `_kind36`, `_other`), the clubs paragraph (`transfer_rumour_clubs`), the agent line
/// (`transfer_rumour_agent_line`) and the links (`transfer_rumour_links`).
/// Original: `Media::write_transfer_rumour_news_text` (0x755860, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_rumour_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut f = [0u32; 0x2c];
    f[0x50 / 4] = slot(0);
    f[0x4c / 4] = slot(1);
    f[0x44 / 4] = slot(2);
    f[0x40 / 4] = slot(3);
    let club = slot(4) as i32;
    let lead = slot(5) as i32;
    f[0x5c / 4] = slot(6);
    f[0x64 / 4] = slot(7);
    f[0x48 / 4] = slot(8);
    let position = slot(9);
    f[0x3c / 4] = slot(10);
    f[0x60 / 4] = slot(11);
    f[0x58 / 4] = slot(12);
    f[0x54 / 4] = slot(13);
    let flag14 = slot(14) as u8;
    let mut count = slot(15) as i32;
    if count > 0 {
        let (mut k, mut i) = (0x10u32, 0usize);
        loop {
            if k == 0x32 {
                count = 0; // the list is too long: no clubs
                break;
            }
            f[0x28 / 4 + i] = slot(k);
            k += 1;
            i += 1;
            if i as i32 >= count {
                break;
            }
        }
    }
    let mut r = Rumour { f, club, lead, position, flag14, count, nation: 0, local: false, desc_gender: 0 };
    let mut desc = [0u8; 0xc8];
    let g = write_position_name(r.position, desc.as_mut_ptr() as usize, 0xc8);
    // the original pushes a seventh argument (1) that the description does not read
    r.desc_gender = write_player_description(r.player(), r.position, r.qualities(), desc.as_mut_ptr() as usize, 0xc8, g as u32) as i8 as i32;
    if r.club > -1 {
        let sum = r.count.wrapping_add(r.agent()).wrapping_add(r.lead).wrapping_add(r.club).wrapping_add(r.player());
        r.nation = match sum % 6 {
            1 | 2 => club_at(r.club as isize).nation().addr(),
            3 => {
                let n = staff_at(r.player() as isize).nation();
                if n.reputation() < 0x1770 { 0 } else { n.addr() } // nma5: the player's nation from 6000
            }
            4 | 5 => club_at(r.listed(0) as isize).nation().addr(),
            _ => 0,
        };
    }
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    if !manager.is_null() {
        r.local = if r.nation == 0 {
            true
        } else if !manager.club_job().is_null() {
            manager.club_job().nation().addr() == r.nation
        } else {
            manager.nation().addr() == r.nation
        };
    }
    if body == 0 {
        transfer_rumour_headline(&r, out);
        return;
    }
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x78];
    let mut player_short = [0u8; 0x78];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x78, 1, r.player() as u32);
    exe::database_format_name_from_ids(n1, n2, n3, player_short.as_mut_ptr(), 0x78, 5, r.player() as u32);
    let mut agent = [0u8; 0x78];
    let mut agent_short = [0u8; 0x78];
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        exe::database_format_name_from_ids(a1, a2, a3, agent.as_mut_ptr(), 0x78, 1, r.agent() as u32);
        exe::database_format_name_from_ids(a1, a2, a3, agent_short.as_mut_ptr(), 0x78, 5, r.agent() as u32);
    }
    let fee_value = r.fee();
    let mut fee = [0u8; 0x32];
    let amount = exe::cash_get_display_amount(&fee_value, 6);
    let format_money: unsafe extern "C" fn(usize, u32, i64, u32) -> i32 = core::mem::transmute(0x441c10usize);
    format_money(fee.as_mut_ptr() as usize, 0x32, amount, 6);
    let (p, fp, d) = (player.as_ptr() as usize, fee.as_ptr() as usize, desc.as_ptr() as usize);
    let piece = match r.lead {
        0 => transfer_rumour_lead_kind0(&r, out, p, fp),
        1 => transfer_rumour_lead_kind1(&r, out, p, fp),
        0x24 => transfer_rumour_lead_kind36(&r, out, p, fp, d),
        _ => transfer_rumour_lead_other(&r, out, p, fp),
    };
    transfer_rumour_clubs(&r, out, piece, d);
    transfer_rumour_agent_line(&r, out, player_short.as_ptr() as usize, agent.as_ptr() as usize);
    transfer_rumour_links(&r, out, p, player_short.as_ptr() as usize, agent.as_ptr() as usize);
}

/// The headline of the transfer rumour: the player's short name (style 5) with "unsettled" / "fuels transfer talk"
/// (lead 36: by the player's id even or odd; "eyes ... move" when there is no bidding club and the first listed club
/// is a club), otherwise by the player's id % 3: 0 "transfer speculation", 1 "rumours" (more than one club) or
/// "rumour", 2 "... lead chase for ..." (more than one club, the first one's short name), "... linked with ..." (one
/// club) or "rumour". The name links to the player (`exe::database_staff_matches_names`, type 0x7d7), the bidding
/// club and every listed club (type 0x7d5).
unsafe fn transfer_rumour_headline(r: &Rumour, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour headline");
    TRANSFER_RUMOUR_PIECE.store(0, Relaxed);
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x190];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x190, 5, r.player() as u32);
    let p = player.as_ptr() as usize;
    let first = r.listed(0);
    if r.lead == 0x24 {
        if r.club == -1 && first > r.club {
            let sg = rumour_club_short_gender(first);
            format(RUMOUR_TEXT, 0xa53c94, sg, sg, p, rumour_club_short(first)); // "... eyes ... move"
        } else if r.player() % 2 != 0 {
            format(RUMOUR_TEXT, 0xa53c58, p); // "... fuels transfer talk"
        } else {
            format(RUMOUR_TEXT, 0xa53c24, p); // "... unsettled"
        }
    } else {
        match r.player() % 3 {
            0 => {
                format(RUMOUR_TEXT, 0xa53ae0, p); // "... transfer speculation"
            }
            1 => {
                format(RUMOUR_TEXT, if r.count > 1 { 0xa53b1c } else { 0xa53b4c }, p); // "... rumours" / "... rumour"
            }
            _ => {
                let sg = rumour_club_short_gender(first);
                if r.count > 1 {
                    format(RUMOUR_TEXT, 0xa53bd0, sg, sg, rumour_club_short(first), p); // "... lead chase for ..."
                } else if r.count == 1 {
                    format(RUMOUR_TEXT, 0xa53b7c, sg, sg, p, rumour_club_short(first)); // "... linked with ..."
                } else {
                    format(RUMOUR_TEXT, 0xa53b4c, p);
                }
            }
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, p as *const u8, 0x7d7, person); // nma5: the headline's player link
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The body lead of kind 0: the first listed club (without a bidding club), the bidding club and the first listed
/// club, or both with the fee. Returns the piece (1).
unsafe fn transfer_rumour_lead_kind0(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 0");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5441c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54398, g4, g4, g0, g0, n4, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54300, g4, g4, g0, g0, n4, player, fee, n0);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    1
}

/// The body lead of kind 1: the first listed club and the player (a foreign manager's lead names the nation by its
/// short name), with the bidding club, and with the fee (then the foreign lead uses the nationality adjective, form
/// 0x1c). Returns the piece (2).
unsafe fn transfer_rumour_lead_kind1(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 1");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let nation = Nation(r.nation);
    if r.fee() == 0 {
        if r.club == -1 {
            if r.local {
                format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa54a6c, sg, sg, g0, g0, nation.addr() + nation::SHORT_NAME, n0, player);
            }
        } else {
            let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
            if r.local {
                format(RUMOUR_TEXT, 0xa549ac, g0, g0, g4, g4, n0, n4, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa548d0, sg, sg, g0, g0, g4, g4, nation.addr() + nation::SHORT_NAME, n0, n4, player);
            }
        }
    } else if r.club == -1 {
        format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
    } else if r.local {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54808, g0, g0, g4, g4, n0, n4, player, fee);
    } else {
        let mut adjective = [0u8; 0x78];
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, 0x1c);
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54724, 0x64, 0x64, g0, g0, g4, g4, adjective.as_ptr() as usize, n0, n4, player, fee);
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    2
}

/// The body lead of kind 36: the first listed club and the player (with his description and its gender when there is
/// a bidding club, and the fee when there is one). Returns the piece (3).
unsafe fn transfer_rumour_lead_kind36(r: &Rumour, out: usize, player: usize, fee: usize, desc: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 36");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let gd = r.desc_gender;
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5504c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54f7c, g4, g4, gd, gd, g0, g0, n4, desc, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54e84, g4, g4, gd, gd, g0, g0, n4, desc, player, n0, fee);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    3
}

/// The body lead of the other kinds: like kind 0 with other texts. Returns the piece (4).
unsafe fn transfer_rumour_lead_other(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead other");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa55578, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa554a4, 0x64, g0, g0, g4, g4, player, n0, n4);
        } else {
            format(RUMOUR_TEXT, 0xa553f4, 0x64, g4, g4, g0, g0, player, fee, n4, n0); // nma5: the fee in the lead
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    4
}

/// The clubs paragraph of the body: with 2 to 5 listed clubs, the 2nd to the 5th by name (the texts by the lead
/// piece; with 2 clubs, kind 36 adds the player's description and the other kinds, for a foreign manager, the
/// nationality adjective, form 0x18), appended to the text.
unsafe fn transfer_rumour_clubs(r: &Rumour, out: usize, piece: u8, desc: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if !(2..=5).contains(&r.count) {
        TRANSFER_RUMOUR_PIECE.store(piece as u32, Relaxed);
        return;
    }
    covered!("nma_5 rumour clubs");
    TRANSFER_RUMOUR_PIECE.store(piece as u32 | (r.count as u32) << 8, Relaxed);
    let g = |i: usize| rumour_club_gender(r.listed(i));
    let n = |i: usize| rumour_club_name(r.listed(i));
    // the texts of 3, 4 and 5 clubs by piece (kind 0, kind 1, kind 36, other)
    let (t3, t4, t5) = match piece {
        1 => (0xa54224, 0xa54188, 0xa540c8),
        2 => (0xa54628, 0xa54578, 0xa544a4),
        3 => (0, 0xa54c90, 0xa54ba0),
        _ => (0xa55240, 0xa55194, 0xa550c0),
    };
    let gd = r.desc_gender;
    match r.count {
        2 => match piece {
            1 => {
                format(RUMOUR_TEXT, 0xa542a4, g(1), g(1), n(1));
            }
            2 => {
                format(RUMOUR_TEXT, 0xa546b8, g(1), g(1), n(1));
            }
            3 => {
                format(RUMOUR_TEXT, 0xa54df4, g(1), g(1), gd, gd, n(1), desc);
            }
            _ => {
                if r.local {
                    format(RUMOUR_TEXT, 0xa55370, g(1), g(1), n(1));
                } else {
                    let mut adjective = [0u8; 0x78];
                    exe::nation_get_nationality_adjective(r.nation, adjective.as_mut_ptr() as usize, 0x78, 0x18);
                    format(RUMOUR_TEXT, 0xa552cc, 0x64, g(1), g(1), adjective.as_ptr() as usize, n(1));
                }
            }
        },
        3 => {
            if piece == 3 {
                format(RUMOUR_TEXT, 0xa54d4c, gd, gd, g(1), g(1), g(2), g(2), desc, n(1), n(2));
            } else {
                format(RUMOUR_TEXT, t3, g(1), g(1), g(2), g(2), n(1), n(2));
            }
        }
        4 => {
            format(RUMOUR_TEXT, t4, g(1), g(1), g(2), g(2), g(3), g(3), n(1), n(2), n(3));
        }
        _ => {
            format(RUMOUR_TEXT, t5, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), n(1), n(2), n(3), n(4));
        }
    }
    rumour_append(out, RUMOUR_TEXT);
}

/// The agent line of the body, appended to the text (empty for the other kinds): when the agent is the human manager
/// of the current screen, by the agent line kind (slot 7): 1 the player's own words (two texts by the flag of slot
/// 14), 2 three texts (the flag, then without a fee and with a fee), 3 a fixed text; otherwise 1 and 2 quote the
/// agent (his full name) on the bidding club (its short name).
unsafe fn transfer_rumour_agent_line(r: &Rumour, out: usize, player_short: usize, agent: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut line = [0u8; 0x190];
    let l = line.as_mut_ptr() as usize;
    let human = r.agent() > -1
        && staff_at(r.agent() as isize).addr() == read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human {
        covered!("nma_5 rumour agent human");
        TRANSFER_RUMOUR_PIECE.fetch_or(1 << 16, Relaxed);
        match r.agent_kind() {
            1 => {
                if r.flag14 != 0 {
                    format(l, 0xa53e90, 0x64, player_short);
                } else {
                    format(l, 0xa53e20, player_short);
                }
            }
            2 => {
                if r.flag14 != 0 {
                    format(l, 0xa54038, player_short);
                } else if r.fee() == 0 {
                    format(l, 0xa53fb8, player_short);
                } else {
                    format(l, 0xa53f38, player_short);
                }
            }
            3 => {
                format(l, 0xa54094);
            }
            _ => {}
        }
    } else {
        covered!("nma_5 rumour agent quoted");
        TRANSFER_RUMOUR_PIECE.fetch_or(2 << 16, Relaxed);
        let k = r.agent_kind();
        if k == 1 || k == 2 {
            let sg = rumour_club_short_gender(r.club);
            format(l, if k == 1 { 0xa53ce0 } else { 0xa53d7c }, sg, sg, rumour_club_short(r.club), agent, player_short);
        }
    }
    rumour_append(out, l);
}

/// The links of the body: the player's two names (when `exe::database_staff_matches_names`; type 0x7d7), the agent's
/// full name (the same), the bidding club (type 0x7d5); with a nation, its name, short name and the ten forms of its
/// nationality adjective (0x16 to 0x1f) to its national team (`exe::database_get_national_team`); every listed club.
unsafe fn transfer_rumour_links(r: &Rumour, out: usize, player: usize, player_short: usize, agent: usize) {
    covered!("nma_5 rumour links");
    let (n1, n2, n3) = r.player_names();
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, player as *const u8, 0x7d7, person);
        exe::text_add_link(out, player_short as *const u8, 0x7d7, person);
    }
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        let a = staff_at(r.agent() as isize);
        if exe::database_staff_matches_names(a, a1, a2, a3, 0usize) != 0 {
            exe::text_add_link(out, agent as *const u8, 0x7d7, a);
        }
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    if r.nation != 0 {
        let n = r.nation;
        exe::text_add_link(out, (n + 4) as *const u8, 0x7d5, exe::database_get_national_team(n)); // the nation's name (+4)
        exe::text_add_link(out, (n + nation::SHORT_NAME) as *const u8, 0x7d5, exe::database_get_national_team(n));
        let mut adjective = [0u8; 0x78];
        for form in 0x16..=0x1fu32 {
            exe::nation_get_nationality_adjective(n, adjective.as_mut_ptr() as usize, 0x78, form);
            exe::text_add_link(out, adjective.as_ptr(), 0x7d5, exe::database_get_national_team(n));
        }
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc; walked like the copy constructor 0x4ae5c0 does).
unsafe fn nma5_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
    let text = |p: usize| if p == 0 { Vec::new() } else { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec() };
    let mut links = Vec::new();
    let l = read_usize(t + 4);
    if l != 0 {
        list_rewind(l);
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            links.push((text(e), read_u32(e + 0xc8), read_u32(e + 0xcc)));
        }
    }
    (text(read_usize(t)), links)
}

/// Comparer for `write_transfer_rumour_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator, without counting the copy's draws); the strings and the links (text, type and
/// data) must be equal. The message names the piece that ran (headline; the lead of kind 0, 1, 36 or other, the
/// clubs paragraph and its number of clubs, the agent line).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_rumour_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_TRANSFER_RUMOUR_NEWS_TEXT);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out),
        || crate::rng::without_counting(|| write_transfer_rumour_news_text(this, news, body, lp)));
    let (mo, mr) = (nma5_text_contents(out), nma5_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    let piece = TRANSFER_RUMOUR_PIECE.load(Relaxed);
    SCOREBOARD.record(same_rng && mo == mr, || {
        let lead = ["headline", "lead kind 0", "lead kind 1", "lead kind 36", "lead other"][(piece & 0xff).min(4) as usize];
        let agent = ["", ", agent line human", ", agent line quoted"][((piece >> 16) & 3).min(2) as usize];
        format!("write_transfer_rumour_news_text(body {}) piece: {}, clubs {}{}; original {:?} {:x?} rust {:?} {:x?}", body, lead, (piece >> 8) & 0xff, agent,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}

/// The random draw of slot 1 of the team story by kind (byte table 0x74bb38, kinds 0x3c to 0x63, jumps 0x74bb20): 0
/// draw(3), 1 draw(4), 2 draw(2) + 1 for a club above reputation 5250 or draw(3), 3 draw(3) and 4 draw(2) (a non-zero
/// draw drops the person), 5 the others (the kind becomes 0x63, a draw(2) that drops the person when non-zero, slot 0).
const TEAM_STORY_DRAW_CASE: [u8; 0x28] = [0, 1, 2, 3, 4, 4, 3, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 3, 0, 3, 3, 1, 3, 3, 3, 3, 3];

/// The kinds of team story the manager answers (byte table 0x74bb68, kinds 0x3e to 0x5e; jumps 0x74bb60 and
/// 0x74bba4): 0 for 0x3e, 0x50, 0x51 and 0x5e (no answer), 1 for the others (and every kind outside the table).
const TEAM_STORY_ANSWERED: [u8; 0x21] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

/// The club jobs the team story shows in its person slot (byte table 0x74bb94, jobs 1 to 15): all but 7, 9, 10, 11
/// and 14 (shown as −1).
const TEAM_STORY_JOB_HIDDEN: [u8; 0xf] = [0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0];

/// The comparer's rehearsal of `post_team_media_story`: the copy runs without delivering the news item, without
/// logging the story and without the manager's answer, and keeps what it would do.
pub(crate) static TEAM_STORY_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static TEAM_STORY_DELIVERIES: AtomicU8 = AtomicU8::new(0);
pub(crate) static TEAM_STORY_LOGGED: AtomicBool = AtomicBool::new(false);
pub(crate) static TEAM_STORY_LOG_PERSON: AtomicUsize = AtomicUsize::new(0);
pub(crate) static TEAM_STORY_LOG_KIND: AtomicU32 = AtomicU32::new(0);
pub(crate) static TEAM_STORY_LOG_REPLY: AtomicU32 = AtomicU32::new(0);
pub(crate) static mut TEAM_STORY_BUILT: [u8; 0xe0] = [0; 0xe0];

/// Delivers the team story `n` through `G_NEWS`: 1 to the club's manager, 2 to the continent's managers, 3 to the
/// division's, 4 to the nation's, 5 to all; in the rehearsal it only counts the delivery and keeps the news item.
unsafe fn team_story_deliver(n: usize, how: u8, target: usize) {
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_DELIVERIES.fetch_add(1, Relaxed);
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(TEAM_STORY_BUILT) as *mut u8, 0xe0);
        return;
    }
    let news = read_usize(G_NEWS);
    match how {
        1 => exe::news_send_to_club_manager(news, n, target),
        2 => exe::news_send_to_continent_managers(news, n, target),
        3 => exe::news_send_to_division_managers(news, n, target),
        4 => exe::news_send_to_nation_managers(news, n, target),
        _ => exe::news_send_to_all_managers(news, n),
    };
}

/// `Media::post_team_media_story(club, kind, person, game, variant, extra)`: posts a media story (news 0xbdf) about a
/// club or national team, logs it and lets a computer manager answer it.
///
/// Nothing before the news start date (`before_news_start_date`), without a club, without a division (unless a
/// national team, `exe::database_is_national_team`), without a nation, without a stadium or its city (unless a
/// national team), without a manager or his sheet (`.non_player()`); a national team only for kinds 0x5a and 0x44;
/// nothing if the story is already in the log (`exe::media_has_logged_story`, type 0xbdf with the kind) or the person
/// is a player below reputation 3500. News item 0xbdf (local, `exe::new_newsitem`): 0 = −1; 1 = the draw by kind
/// (`TEAM_STORY_DRAW_CASE`; some kinds drop the person on a non-zero draw, the others become kind 0x63); 2 = the tone:
/// with the club's reputation at most 4500 + draw(1500) and not a national team, 2 (draw(3) ≤ 1) or 4; otherwise by
/// draw(100): 0 → 6, below 8 → 3, below 35 → 5, below 60 → 0, otherwise 1 (national team or draw(3) non-zero) or 2;
/// 3 = the nation's id; 4 = the club's id; 9 = the kind; 10 = the computer manager's answer
/// (`exe::media_pick_ai_manager_response`, for the answered kinds of `TEAM_STORY_ANSWERED` at a non-human club,
/// otherwise 0); 5 and 6 = the manager's ambition above 15 with temperament below 6, or ambition below 15 with
/// temperament above 14, both with reputation above 5750; 7 = the competition (kinds 0x5a/0x44: the nation's current
/// competition, `exe::nationalteams_get_current_competition`, only if a finals one; 0x5c: the continental
/// competition, `club::CONTINENTAL_COMP_ID`, only if there is one; otherwise the division); 0xb-0xe the manager and
/// his names; 0xf-0x12 the person and his names or −1; 0x13 = whether there is a game (then its record,
/// `exe::fill_match_record_from_fixture` and `exe::write_match_record_to_news`, from slot 0x14 on). From the next
/// free slot: the variant, whether the club was the last one placed in the competition
/// (`exe::comp_util_get_latest_placed_club`), the next hosts (`exe::hostcountry_find_next_hosts`), the extra word, the
/// person's club and club job (jobs 7, 9, 10, 11, 14 and those above 15 as −1) or national team and job (not job 11)
/// or −1 −1, whether he has caps, the shoot-out (0 none, 1 won, 2 lost; nothing for a level one) with a game, and a
/// pundit (`exe::punditlist_pick_random_pundit` by the tone and the nation). Delivery: a non-human club, on a
/// draw(2) non-zero, to its manager; otherwise kinds 0x5a/0x44 to all; 0x5c to the continent; a division above
/// reputation 15 with the club at 9000 or more, to the continent; with the club above 8000 + draw(3000) − (30 × the
/// variant + 1000 above variant 20), kinds 0x50/0x51 above variant 15 and 0x41, 0x5d, 0x61, 0x62, 0x63 to the continent;
/// kind 0x3d to the division on a draw(3) of 0, otherwise to the manager; the others to the division (club below
/// 5000) or to the nation (an English club from 6000 also to Scotland, a Scottish one from 6500 to England, the
/// others also to the club's manager). Then the log (`log_story`) and, for an answered kind at a non-human club, the
/// answer (`Media::apply_team_story_response`, 0x73e870).
/// Original: `Media::post_team_media_story` (0x74b060, media.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let club = Club(club);
    let mut person = Staff(person);
    let mut kind = kind;
    let mut record = [0u8; 0x20]; // the match record (a Date at its start)
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    if before_news_start_date() || club.is_null() {
        return;
    }
    let national = || exe::database_is_national_team(club) != 0;
    if club.division().is_null() && !national() {
        return;
    }
    if club.nation().is_null() {
        return;
    }
    if club.stadium().is_null() && !national() {
        return;
    }
    if !national() && club.stadium().city() == 0 {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let k = kind as u8;
    if national() && k != 0x5a && k != 0x44 {
        return;
    }
    if exe::media_has_logged_story(this, person, club, 0xbdf, k as i8 as i16 as u16 as u32, core::ptr::null()) != 0 {
        return;
    }
    if !person.is_null() && !person.player().is_null() && person.player().current_reputation() < 0xdac {
        return;
    }
    covered!("nma_5 team story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbdf, 0);
    exe::newsitem_set_slot(n, 0, u32::MAX);
    let e = (k as i8 as i32).wrapping_sub(0x3c);
    let case = if (0..0x28).contains(&e) { TEAM_STORY_DRAW_CASE[e as usize] } else { 5 };
    let draw = match case {
        0 => exe::utils_random(3),
        1 => exe::utils_random(4),
        2 => {
            if club.reputation() > 0x1482 {
                (exe::utils_random(2) as u8).wrapping_add(1) as i8 as i32
            } else {
                exe::utils_random(3)
            }
        }
        3 | 4 => {
            let d = exe::utils_random(if case == 3 { 3 } else { 2 });
            if d != 0 {
                person = Staff(0);
            }
            d
        }
        _ => {
            covered!("nma_5 team story other kind");
            kind = (kind & !0xff) | 0x63;
            if exe::utils_random(2) != 0 {
                person = Staff(0);
            }
            0
        }
    };
    let k = kind as u8 as i8;
    exe::newsitem_set_slot(n, 1, draw as u32);
    let tone: u32 = if (club.reputation() as i32) > exe::utils_random(0x5dc) + 0x1194 || national() {
        let d = exe::utils_random(100) as u8 as i8;
        if d == 0 {
            6
        } else if d < 8 {
            3
        } else if d < 0x23 {
            5
        } else if d < 0x3c {
            0
        } else if national() || exe::utils_random(3) != 0 {
            1
        } else {
            2
        }
    } else if (0..=1).contains(&exe::utils_random(3)) {
        2
    } else {
        4
    };
    exe::newsitem_set_slot(n, 2, tone);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 9, k as i32 as u32);
    let a = (k as i32).wrapping_sub(0x3e);
    let answered = !((0..0x21).contains(&a) && TEAM_STORY_ANSWERED[a as usize] == 0);
    let reply = if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        exe::media_pick_ai_manager_response(this, manager, person, k as i16 as u16 as u32) // nma5: the AI manager's answer
    } else {
        0
    };
    exe::newsitem_set_slot(n, 0xa, reply);
    let rep = manager.non_player().current_reputation();
    exe::newsitem_set_slot(n, 5, (manager.ambition() > 0xf && manager.temperament() < 6 && rep > 0x1676) as u32);
    exe::newsitem_set_slot(n, 6, (manager.ambition() < 0xf && manager.temperament() > 0xe && rep > 0x1676) as u32);
    let k = k as u8;
    let comp = if k == 0x5a || k == 0x44 {
        let nt = read_usize(G_NATIONAL_TEAMS);
        if exe::national_teams_is_finals_comp(nt, exe::nationalteams_get_current_competition(nt, club.nation())) == 0 {
            return;
        }
        read_u32(exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation()))
    } else if k == 0x5c {
        let c = club.continental_comp_id();
        if c == u32::MAX {
            return;
        }
        c
    } else {
        read_u32(club.division().addr())
    };
    exe::newsitem_set_slot(n, 7, comp);
    exe::newsitem_set_slot(n, 0xb, read_u32(manager.addr()));
    exe::newsitem_set_slot(n, 0xc, read_u32(manager.first_name() + names::ID));
    exe::newsitem_set_slot(n, 0xd, read_u32(manager.second_name() + names::ID));
    exe::newsitem_set_slot(n, 0xe, read_u32(manager.common_name() + names::ID));
    if !person.is_null() {
        exe::newsitem_set_slot(n, 0xf, read_u32(person.addr()));
        exe::newsitem_set_slot(n, 0x10, read_u32(person.first_name() + names::ID));
        exe::newsitem_set_slot(n, 0x11, read_u32(person.second_name() + names::ID));
        exe::newsitem_set_slot(n, 0x12, read_u32(person.common_name() + names::ID));
    } else {
        for s in 0xf..=0x12 {
            exe::newsitem_set_slot(n, s, u32::MAX);
        }
    }
    let mut c: u8 = 0x14;
    if game != 0 {
        exe::newsitem_set_slot(n, 0x13, 1);
        exe::fill_match_record_from_fixture(game, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut c);
    } else {
        exe::newsitem_set_slot(n, 0x13, 0);
    }
    let mut put = |v: u32| {
        exe::newsitem_set_slot(n, c as i8, v);
        c = c.wrapping_add(1);
    };
    put(variant as i8 as i32 as u32);
    let latest = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp as usize).wrapping_mul(0x6b)), 0);
    put((latest == club.addr()) as u32);
    let (mut host_a, mut host_b) = (0u32, u32::MAX);
    exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), comp, &mut host_a, &mut host_b);
    put(host_a);
    put(host_b);
    put(extra);
    let caps = if !person.is_null() {
        let cj = person.club_job();
        let nj = person.national_job();
        if !cj.is_null() {
            put(read_u32(cj.addr()));
            let j = person.job_for_club() as i8 as i32;
            put(if (1..=0xf).contains(&j) && TEAM_STORY_JOB_HIDDEN[(j - 1) as usize] == 0 { j as u32 } else { u32::MAX });
        } else if !nj.is_null() && person.job_for_nation() != 0xb {
            put(read_u32(nj.addr()));
            put(person.job_for_nation() as i8 as i32 as u32);
        } else {
            put(u32::MAX);
            put(u32::MAX);
        }
        person.int_apps() != 0
    } else {
        put(u32::MAX);
        put(u32::MAX);
        false
    };
    put(caps as u32);
    if game != 0 {
        let g = Fixture(game);
        let (h, w) = (g.home_penalties(), g.away_penalties());
        if h == -1 {
            put(0);
        } else if h > w {
            put(if g.home_club() == club { 1 } else { 2 });
        } else if h < w {
            put(if g.away_club() == club { 1 } else { 2 });
        }
    }
    let nation = club.nation();
    let pundit = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, tone, if !nation.is_null() { read_i32(nation.addr()) } else { -1 });
    put(pundit as u32);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && exe::utils_random(2) != 0 {
        team_story_deliver(n, 1, club.addr());
    } else {
        team_story_deliver_by_kind(n, club, k, variant as u8 as i8);
    }
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_LOGGED.store(true, Relaxed);
        TEAM_STORY_LOG_PERSON.store(person.addr(), Relaxed);
        TEAM_STORY_LOG_KIND.store(kind, Relaxed);
        TEAM_STORY_LOG_REPLY.store(reply, Relaxed);
        return;
    }
    log_story(this, person.addr(), club.addr(), 0xbdf, kind, reply);
    if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        let respond: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(0x73e870usize);
        respond(this, club.addr(), k as i8 as i16 as u16 as u32, reply, person.addr());
    }
}

/// The delivery of the team story when it does not go to the club's manager alone (see `post_team_media_story`).
unsafe fn team_story_deliver_by_kind(n: usize, club: Club, k: u8, variant: i8) {
    if k == 0x5a || k == 0x44 {
        covered!("nma_5 team story to all");
        return team_story_deliver(n, 5, 0);
    }
    let continent = club.nation().continent();
    if k == 0x5c {
        return team_story_deliver(n, 2, continent);
    }
    let div = club.division();
    if !div.is_null() && club.reputation() >= 0x2328 && div.reputation() > 0xf {
        return team_story_deliver(n, 2, continent);
    }
    if !div.is_null() && div.reputation() > 0xf {
        let bonus = if variant > 0x14 { (variant as i32) * 30 + 1000 } else { 0 };
        if (club.reputation() as i32) > exe::utils_random(0xbb8) + 0x1f40 - bonus {
            if k == 0x50 || k == 0x51 {
                if variant > 0xf {
                    return team_story_deliver(n, 2, continent);
                }
                return team_story_deliver_in_country(n, club, div);
            }
            if [0x41, 0x62, 0x63, 0x61, 0x5d].contains(&k) {
                return team_story_deliver(n, 2, continent);
            }
        }
    }
    if k == 0x3d {
        if div.is_null() || exe::utils_random(3) != 0 {
            return team_story_deliver(n, 1, club.addr());
        }
        return team_story_deliver(n, 3, div.addr());
    }
    team_story_deliver_in_country(n, club, div)
}

/// The team story inside the club's country: without a division to the club's manager; below reputation 5000 to the
/// division's managers; otherwise to the nation's managers, and then an English club from 6000 also to the Scottish
/// managers, a Scottish one from 6500 to the English ones, the others to the club's manager.
unsafe fn team_story_deliver_in_country(n: usize, club: Club, div: Competition) {
    if div.is_null() {
        return team_story_deliver(n, 1, club.addr());
    }
    if club.reputation() < 0x1388 {
        return team_story_deliver(n, 3, div.addr());
    }
    covered!("nma_5 team story to the nation");
    team_story_deliver(n, 4, club.nation().addr());
    let id = read_u32(club.nation().addr());
    let nation_record = |key: usize| read_usize(G_NATION_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(nation::SIZE));
    if id == read_u32(KEY_NATION_ENGLAND) && club.reputation() >= 0x1770 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_SCOTLAND));
    } else if id == read_u32(KEY_NATION_SCOTLAND) && club.reputation() >= 0x1964 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_ENGLAND));
    } else {
        team_story_deliver(n, 1, club.addr());
    }
}

/// Comparer for `post_team_media_story`: the copy runs as a rehearsal (without delivering, logging or answering) and
/// the original from the same random state. Must be equal: the copy's random draws are the beginning of the
/// original's (the deliveries' automatic replies and the answer draw afterwards); the original logged the story
/// (one more record of type 0xbdf for the club, the person, the kind and the answer, dated today) exactly when the
/// copy would; and each human manager whose inbox (News +8: 12 bytes per human) advanced received the copy's news
/// item (type +0..4 and data +5..0xd6); with no delivery planned, no inbox advances.
pub(crate) unsafe extern "thiscall" fn compare_post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32)>(IDX_POST_TEAM_MEDIA_STORY);
    let news = read_usize(G_NEWS);
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    let boxes_before: Vec<(i32, i32)> = if boxes != 0 { (0..0x10).map(|k| (read_i32(boxes + 12 * k), read_i32(boxes + 12 * k + 4))).collect() } else { Vec::new() };
    let log_before = read_i32(this + media_fields::STORY_LOG_COUNT);
    let before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    TEAM_STORY_DRY_RUN.store(true, Relaxed);
    TEAM_STORY_DELIVERIES.store(0, Relaxed);
    TEAM_STORY_LOGGED.store(false, Relaxed);
    let (_, rr) = crate::rng::trace(|| crate::rng::without_counting(|| post_team_media_story(this, club, kind, person, game, variant, extra)));
    TEAM_STORY_DRY_RUN.store(false, Relaxed);
    let (deliveries, logged, built) = (TEAM_STORY_DELIVERIES.load(Relaxed), TEAM_STORY_LOGGED.load(Relaxed), *core::ptr::addr_of!(TEAM_STORY_BUILT));
    before.write_back();
    let (_, ro) = crate::rng::trace(|| o(this, club, kind, person, game, variant, extra));
    // the log: a record of this story, dated today, exactly when the copy logs one
    let mut found = false;
    let log_after = read_i32(this + media_fields::STORY_LOG_COUNT);
    if logged {
        let p = TEAM_STORY_LOG_PERSON.load(Relaxed);
        let want_person = if p != 0 { read_u32(p) } else { u32::MAX };
        for i in 0..log_after.max(0) as usize {
            let e = read_usize(this).wrapping_add(i * 0x17);
            if read_u32(e + story_log_entry::TYPE) == 0xbdf && read_u32(e + story_log_entry::PERSON_B) == read_u32(club)
                && read_u32(e + story_log_entry::PERSON_A) == want_person
                && read_i16(e + story_log_entry::SUBTYPE) == TEAM_STORY_LOG_KIND.load(Relaxed) as u8 as i8 as i16
                && read_u8(e + story_log_entry::RESPONSE) == TEAM_STORY_LOG_REPLY.load(Relaxed) as u8
                && read_u32(e) == read_u32(G_CURRENT_DATE)
            {
                found = true;
                break;
            }
        }
    }
    let log_same = if logged { found && log_after > log_before } else { log_after == log_before };
    // the human managers' inboxes
    let (mut delivered, mut news_same) = (0, true);
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    if boxes != 0 && boxes_before.len() == 0x10 {
        for (k, &(start, next)) in boxes_before.iter().enumerate() {
            let e = boxes + 12 * k;
            if read_i32(e + 4) == next && read_i32(e) == start {
                continue;
            }
            delivered += 1;
            let v = if start == -1 { read_i32(e) } else { next }; // the first news item opens the ring at the start
            let s = read_usize(news).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != built[0..4] || g[5..0xd6] != built[5..0xd6] {
                news_same = false;
            }
        }
    }
    let same = ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && log_same && news_same && (deliveries != 0 || delivered == 0);
    SCOREBOARD.record(same, || format!("post_team_media_story(club {:#x}, kind {:#x}) draws original {:?} rust {:?}; logged {} found {} log {} -> {}; deliveries {} delivered {} news equal {}",
        club, kind as u8, ro, rr, logged, found, log_before, log_after, deliveries, delivered, news_same));
}

// ---------- batch 294: the manager's answer to a media story about a player (media.cpp) ----------

/// The calls of `apply_player_story_response` that have no `exe::` wrapper yet (cm0102.exe 3.9.68): the two confidence
/// adjustments and the apology news are hooked (they reach the active copy through the detour), the story effects and
/// the complaint are still original. Each of them may post news.
#[allow(dead_code)]
pub(crate) mod player_story_fns {
    pub(crate) const ADJUST_SUPPORTER_CONFIDENCE: usize = 0x738340; // Media::adjust_supporter_confidence, thiscall, ret 0x14
    pub(crate) const ADJUST_BOARD_CONFIDENCE: usize = 0x7386f0; // Media::adjust_board_confidence, thiscall, ret 0x18
    pub(crate) const APPLY_STORY_EFFECT_FLAGS: usize = 0x736c00; // Media::apply_story_effect_flags, thiscall, ret 0x1c
    pub(crate) const QUEUE_PLAYER_APOLOGY_NEWS: usize = 0x72bbf0; // media_queue_player_apology_news, stdcall, ret 0xc
    pub(crate) const FILE_COMPLAINT: usize = 0x56a100; // DISPUTE::file_complaint, thiscall, ret 0x10
    pub(crate) const F64_3000: usize = 0x96d200; // f64 = 3000.0 (.rdata)
}

/// Which of the player's complaint tests (0x741f20, the byte table of the jump at 0x741d00) a story kind 0..0x23 uses
/// when the manager answers 2: 0 the general test, 1 the strict test, 2 none.
pub(crate) const PLAYER_STORY_COMPLAINT_CASE: [u8; 0x24] = [
    0, 0, 0, 1, 1, 0, 0, 2, 2, 0, 2, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
];

/// The piece of `apply_player_story_response` a story kind goes to (the comparer's message).
pub(crate) fn player_story_response_piece(topic: u32) -> &'static str {
    match topic as u16 {
        0x25 => "position rival (0x25)",
        0x1f => "transfer interest (0x1f)",
        0x20 => "outburst (0x20)",
        _ => "other story",
    }
}

/// In the comparer's rehearsal (`MEDIA_REACTION_DRY_RUN`) a call that may post news is not made: the rehearsal stops
/// there (`MEDIA_REACTION_STOPPED`) and the comparison falls back on the original alone. Outside it: false.
unsafe fn player_story_stops_before_news() -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    false
}

/// STAFF_CONTRACT::apply_event_flags(marks, 0) (`exe::staff_contract_apply_event_flags`); in the rehearsal it stops
/// first when the person would request a transfer (`would_request_transfer`).
unsafe fn player_story_event_flags_or_stop(c: Contract, marks: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) && would_request_transfer(c, marks) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    exe::staff_contract_apply_event_flags(c, marks, 0);
    false
}

/// A squad member the squad loops of `respond_to_transfer_interest_story` reach: not null, with a player record and an
/// id below the staff count.
unsafe fn is_story_squad_member(p: Staff) -> bool {
    !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
}

/// `Media::apply_story_effect_flags(club, kind, flags, weight, response, 0, player)`: the effects of the story's outcome.
unsafe fn player_story_effects(this: usize, club: Club, kind: u32, flags: u32, weight: f32, response: u32, player: Staff) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, usize) = core::mem::transmute(player_story_fns::APPLY_STORY_EFFECT_FLAGS);
    f(this, club.addr(), kind, flags, weight.to_bits(), response, 0, player.addr())
}

/// `Media::adjust_supporter_confidence(club, player, kind, target, response)`.
unsafe fn player_story_supporters(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_SUPPORTER_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response)
}

/// `Media::adjust_board_confidence(club, player, kind, target, response, 0)`.
unsafe fn player_story_board(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_BOARD_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response, 0)
}

/// Story 0x25 (a position rival arrived: `warn_squad_of_position_rival`), the manager's answer to the player.
///
/// Answer 1: the supporters' confidence (+8 of the club's row in [[`G_MANAGER_MANAGER`]], 1..10000) +200 when the
/// club favours the player (`exe::club_favours_staff`), otherwise −50; with `.professionalism()` below 10 and
/// draw(professionalism) below 5, `.professionalism()` and `.temperament()` each go down by one (above 1 and with a
/// nonzero draw of their own); morale +10 (`exe::staff_adjust_morale`, limited); with a nonzero draw(temperament) and
/// the player disliking the manager (`exe::staff_is_disliked_by`), the manager leaves his disliked list
/// (`exe::staff_remove_disliked_staff`); with a nonzero draw(`.pressure()`), contract flag 0x800 is cleared.
///
/// Answer 2: supporters −50 when the club favours him, otherwise +200; `.professionalism()` above 10 and above
/// draw(20): the apology news (`media_queue_player_apology_news`, story 0x25) and nothing else; professionalism 6 or
/// more: nothing else. Otherwise, with the contract (looked up again) and draw(pressure) below draw(10), contract flag
/// 0x800 (`set_flags`); without it, pressure, determination and the player record's `.work_rate()` each go up by one
/// (below 20, on a zero draw of their own). Then morale −10; with the contract wanting to leave
/// (`exe::staff_contract_wants_to_leave`): loyalty −1 when draw(loyalty) is below draw(25) (above 1); on a zero
/// draw(loyalty), the manager goes into his disliked list (`exe::staff_add_disliked_staff`, forced) and the contract
/// gets event flags 8 and, on a zero draw(pressure), also 0x2000 (`exe::staff_contract_apply_event_flags` with 0).
///
/// Answer 3: supporters −100, board (+6) −50.
unsafe fn respond_to_position_rival_story(this: usize, player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 position rival");
    match response as i8 {
        1 => {
            covered!("nma_6 position rival backed");
            let v = if exe::club_favours_staff(player, club) != 0 { 200 } else { -50 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            if player.professionalism() < 10 && exe::utils_random(player.professionalism() as i32) < 5 {
                if player.professionalism() > 1 && exe::utils_random(player.professionalism() as i32) != 0 {
                    player.set_professionalism(player.professionalism().wrapping_sub(1));
                }
                if player.temperament() > 1 && exe::utils_random(player.temperament() as i32) != 0 {
                    player.set_temperament(player.temperament().wrapping_sub(1));
                }
            }
            exe::staff_adjust_morale(player, 10, 1);
            if exe::utils_random(player.temperament() as i32) != 0 && exe::staff_is_disliked_by(player, manager) != 0 {
                exe::staff_remove_disliked_staff(player, manager);
            }
            if exe::utils_random(player.pressure() as i32) == 0 {
                return;
            }
            if contract.flags() & 0x800 != 0 {
                exe::staff_contract_clear_flags(contract, 0x800);
            }
        }
        2 => {
            covered!("nma_6 position rival criticised");
            let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
            let v = if exe::club_favours_staff(player, club) != 0 { -50 } else { 200 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            let professionalism = player.professionalism();
            if professionalism > 10 && (professionalism as i32) > exe::utils_random(0x14) {
                if player_story_stops_before_news() {
                    return;
                }
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(player_story_fns::QUEUE_PLAYER_APOLOGY_NEWS);
                f(this, player.addr(), club.addr(), 0x25);
                return;
            }
            if professionalism >= 6 {
                return;
            }
            let mut flagged = false;
            if !c.is_null() {
                let a = exe::utils_random(player.pressure() as i32);
                if a < exe::utils_random(10) {
                    if set_contract_flags_or_stop(c.addr(), 0x800) {
                        return;
                    }
                    flagged = true;
                }
            }
            if !flagged {
                if player.pressure() < 0x14 && exe::utils_random(player.pressure() as i32) == 0 {
                    player.set_pressure(player.pressure().wrapping_add(1));
                }
                if player.determination() < 0x14 && exe::utils_random(player.determination() as i32) == 0 {
                    player.set_determination(player.determination().wrapping_add(1));
                }
                let sheet = player.player();
                if sheet.work_rate() < 0x14 && exe::utils_random(sheet.work_rate() as i32) == 0 {
                    sheet.set_work_rate(sheet.work_rate().wrapping_add(1));
                }
            }
            exe::staff_adjust_morale(player, -10i32 as u32, 1);
            if exe::staff_contract_wants_to_leave(c) == 0 {
                return;
            }
            if player.loyalty() > 1 {
                let a = exe::utils_random(player.loyalty() as i32);
                if a < exe::utils_random(0x19) {
                    player.set_loyalty(player.loyalty().wrapping_sub(1));
                }
            }
            if exe::utils_random(player.loyalty() as i32) != 0 {
                return;
            }
            exe::staff_add_disliked_staff(player, manager, 1);
            let r = exe::utils_random(player.pressure() as i32);
            if player_story_event_flags_or_stop(c, 8) {
                return;
            }
            if r == 0 {
                player_story_event_flags_or_stop(c, 0x2000);
            }
        }
        3 => {
            covered!("nma_6 position rival no comment");
            add_clamped_confidence(manager_manager_row(club) + 8, -100);
            add_clamped_confidence(manager_manager_row(club) + 6, -50);
        }
        _ => {}
    }
}

/// Story 0x1f (another club's interest in the player: `process_transfer_interest`), the manager's answer; each answer
/// walks the 50 squad slots (`.squad()`).
///
/// Answer 1 (keeps the player): for the player himself, supporters +200 and board +100 when the club favours him,
/// −100 and −100 when it dislikes him (`exe::club_dislikes_staff`); board +250 when he likes the chairman
/// (`exe::staff_is_favoured_by`), −250 when he dislikes him; morale +10 when his contract does not want to leave;
/// otherwise morale −10 and, with draw(loyalty) below draw(25): loyalty −1 on a zero draw (above 1), the manager into
/// his disliked list (forced) and event flags 8 (and 0x2000 on a zero draw(pressure)). For the others (a player record,
/// a valid id): morale +10 when the player likes them, −5 when he dislikes them, otherwise draw(5) − draw(5).
///
/// Answer 2 (lets him go): for the player, supporters −500 and board −200 when the club favours him, +500 and +200
/// when it dislikes him; board −500 when he likes the chairman, +200 when he dislikes him; morale +10 when his contract
/// wants to leave; otherwise morale −10 and, with draw(loyalty) below draw(25), loyalty −1 on a zero draw (above 1) and
/// the manager into his disliked list. The others: liked, morale professionalism / 2 − 15; disliked, +10; otherwise,
/// with draw(ambition) above draw(20) and draw(professionalism) below draw(20), −5.
///
/// Other answers: morale draw(8) − draw(8) for the player and draw(5) − draw(5) for the others.
unsafe fn respond_to_transfer_interest_story(player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 transfer interest");
    match response as i8 {
        1 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest kept");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 200); // nma6: kept, a favourite: supporters +200
                        add_clamped_confidence(manager_manager_row(club) + 6, 100);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -100);
                        add_clamped_confidence(manager_manager_row(club) + 6, -100);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 250);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -250);
                    }
                    if exe::staff_contract_wants_to_leave(contract) == 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1); // nma6: kept against his will: he dislikes the manager
                    let r = exe::utils_random(player.pressure() as i32);
                    if player_story_event_flags_or_stop(contract, 8) {
                        return;
                    }
                    if r == 0 && player_story_event_flags_or_stop(contract, 0x2000) {
                        return;
                    }
                } else if is_story_squad_member(p) {
                    let delta = if exe::staff_is_favoured_by(player, p) != 0 {
                        10
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        -5
                    } else {
                        let a = exe::utils_random(5);
                        (a as u8).wrapping_sub(exe::utils_random(5) as u8) as i8
                    };
                    exe::staff_adjust_morale(p, delta as u32, 1);
                }
            }
        }
        2 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest released");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -500); // nma6: released, a favourite: supporters -500
                        add_clamped_confidence(manager_manager_row(club) + 6, -200);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 500);
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -500);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_contract_wants_to_leave(contract) != 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1);
                } else if is_story_squad_member(p) {
                    if exe::staff_is_favoured_by(player, p) != 0 {
                        let half = (p.professionalism() as i32 / 2) as u8;
                        exe::staff_adjust_morale(p, half.wrapping_sub(0xf) as i8 as u32, 1);
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        exe::staff_adjust_morale(p, 10, 1);
                    } else {
                        let a = exe::utils_random(p.ambition() as i32);
                        if a > exe::utils_random(0x14) {
                            let b = exe::utils_random(p.professionalism() as i32);
                            if b < exe::utils_random(0x14) {
                                exe::staff_adjust_morale(p, -5i32 as u32, 1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            covered!("nma_6 transfer interest other answer");
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                let range = if p == player {
                    8
                } else if is_story_squad_member(p) {
                    5 // nma6: the squad's mood after the rumour
                } else {
                    continue;
                };
                let a = exe::utils_random(range);
                let delta = (a as u8).wrapping_sub(exe::utils_random(range) as u8);
                exe::staff_adjust_morale(p, delta as i8 as u32, 1);
            }
        }
    }
}

/// Story 0x20 (the player's outburst, `create_player_outburst_story` [inference]), the manager's answer.
///
/// Answer 1: supporters' target 500 when the club favours the player, −1500 when it dislikes him, otherwise −500
/// (`adjust_supporter_confidence`); board's target (`adjust_board_confidence`) trunc(w × 1000 − 2000) when he likes the
/// chairman, trunc(w × −1000 − 2000) when he dislikes him, otherwise −2000, where w = (chairman's `.ambition()` − 10)
/// × 0.05 + 1 (0.5 without a chairman); then the story effects with flags 0x100. Answer 2: supporters' target 3000 when
/// the club dislikes him, board's target trunc(w × 3000) when he dislikes the chairman; then the story effects with
/// flags 0x80. Other answers: nothing.
unsafe fn respond_to_outburst_story(this: usize, player: Staff, club: Club, chairman_weight: f64, stat: f32, response: u32) {
    covered!("nma_6 outburst");
    match response as i8 {
        1 => {
            if player_story_stops_before_news() {
                return;
            }
            let target = if exe::club_favours_staff(player, club) != 0 {
                500
            } else if exe::club_dislikes_staff(player, club) != 0 {
                -1500
            } else {
                -500
            };
            player_story_supporters(this, club, player, 0x20, target, response);
            let board = if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_1000) - read_f64(rdata::F64_2000))
            } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_NEG_1000) - read_f64(rdata::F64_2000))
            } else {
                -2000
            };
            player_story_board(this, club, player, 0x20, board, response);
            player_story_effects(this, club, 0x20, 0x100, stat, response, player);
        }
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            if exe::club_dislikes_staff(player, club) != 0 {
                player_story_supporters(this, club, player, 0x20, 3000, response);
            }
            if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                let board = ftol(chairman_weight * read_f64(player_story_fns::F64_3000));
                player_story_board(this, club, player, 0x20, board, response);
            }
            player_story_effects(this, club, 0x20, 0x80, stat, response, player);
        }
        _ => {}
    }
}

/// The player's complaint test of answer 2 to the other stories (`PLAYER_STORY_COMPLAINT_CASE`): a player record with
/// `.current_reputation()` above 6750, `.world_reputation()` above 3750 (the general test also takes a contract
/// `.happiness()` of 100), `.current_ability()` above 120, aged 20 to 33, `.ambition()` above 10, the contract with
/// flags or grievances, its `.morale()` below −25 and wanting to leave, and a zero draw(loyalty). The general test asks
/// `.professionalism()` below 10, `.loyalty()` below 6 and `.temperament()` below 10; the strict one,
/// `.professionalism()` below 6, `.pressure()` above 10 and loyalty or temperament below 6.
unsafe fn player_story_complaint_due(player: Staff, contract: Contract, age: i8, strict: bool) -> bool {
    let sheet = player.player();
    if sheet.is_null() || sheet.current_reputation() <= 0x1a5e {
        return false;
    }
    if sheet.world_reputation() <= 0xea6 && (strict || contract.happiness() != 0x64) {
        return false;
    }
    if sheet.current_ability() <= 0x78 || age <= 0x13 || age >= 0x22 || player.ambition() <= 10 {
        return false;
    }
    let character = if strict {
        player.professionalism() < 6 && player.pressure() > 10 && (player.loyalty() < 6 || player.temperament() < 6)
    } else {
        player.professionalism() < 10 && player.loyalty() < 6 && player.temperament() < 10
    };
    if !character {
        return false;
    }
    if contract.flags() == 0 && contract.grievances() == 0 {
        return false;
    }
    if contract.morale() >= -25 || exe::staff_contract_wants_to_leave(contract) == 0 {
        return false;
    }
    exe::utils_random(player.loyalty() as i32) == 0
}

/// The other stories (neither 0x25, 0x1f nor 0x20): the story effects (`Media::apply_story_effect_flags`, weight = the
/// club's stat 3 in its division) with flags 0x500 for answer 1, 0x404 for answer 3 and 0x480 for answer 2; with
/// answer 2, the player of a kind 0..0x23 that passes his complaint test (`player_story_complaint_due`) first files a
/// complaint (`DISPUTE::file_complaint`, reason 4, and 1 on a zero draw(4)). Other answers: nothing.
#[allow(clippy::too_many_arguments)]
unsafe fn respond_to_other_player_story(this: usize, player: Staff, club: Club, contract: Contract, age: i8, stat: f32, topic: u32, response: u32) {
    covered!("nma_6 other story");
    let flags = match response as i8 {
        1 => 0x500, // nma6: the flags of answer 1
        3 => 0x404,
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            let k = topic as i16 as i32 as u32;
            let case = if k > 0x23 { 2 } else { PLAYER_STORY_COMPLAINT_CASE[k as usize] };
            if case != 2 && player_story_complaint_due(player, contract, age, case == 1) {
                covered!("nma_6 other story complaint");
                let reason_b = (exe::utils_random(4) == 0) as u32;
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(player_story_fns::FILE_COMPLAINT);
                f(read_usize(G_DISPUTE), player.addr(), club.addr(), 4, reason_b);
            }
            0x480
        }
        _ => return,
    };
    if player_story_stops_before_news() {
        return;
    }
    player_story_effects(this, club, topic, flags, stat, response, player);
}

/// `Media::apply_player_story_response(player, club, topic, response)`: the effects of the manager's answer (1, 2, 3)
/// to a media story about one of his players.
///
/// Nothing without the player, the club, the club's manager (+0xcf) with a staff sheet (`.non_player()`), the player's
/// contract (`exe::contract_manager_get_contract`) or, for a club that is not a national team
/// (`exe::database_is_national_team`), its division's object in `G_COMP_OBJECT_TABLE` with the stats (+8). The weight of
/// the story effects is the club's stat 3 there (`exe::comp_stats_get_club_stat`, as a float; 0 for a national team).
/// Then by topic: 0x25 `respond_to_position_rival_story`, 0x1f `respond_to_transfer_interest_story`, 0x20
/// `respond_to_outburst_story`, the rest `respond_to_other_player_story`.
///
/// Called by the computer manager's answer to a new story (`create_player_media_story`, 0x72d1ed), the outburst
/// (0x733f97, answer 0), the human manager's answer screen (0x743275), the praise story (0x744f52) and the transfer
/// rumour (`process_transfer_interest`, 0x7551dd).
/// Original: `Media::apply_player_story_response` (0x741020, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let (player, club) = (Staff(player), Club(club));
    if club.is_null() || player.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    if contract.is_null() {
        return;
    }
    let comp_object = |c: Club| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(c.division().addr())));
    if exe::database_is_national_team(club) == 0 && (club.division().is_null() || comp_object(club) == 0) {
        return;
    }
    let mut stats = 0usize;
    if exe::database_is_national_team(club) == 0 {
        stats = read_usize(comp_object(club) + 8);
        if stats == 0 {
            return;
        }
    }
    let age = player.age() as i8;
    let chairman = club.chairman();
    let chairman_weight = if chairman.is_null() {
        0.5
    } else {
        ((chairman.ambition() as i32) - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let stat = if stats != 0 { exe::comp_stats_get_club_stat(stats, club, 3) as f32 } else { 0.0 };
    match topic as u16 {
        0x25 => respond_to_position_rival_story(this, player, club, manager, contract, response),
        0x1f => respond_to_transfer_interest_story(player, club, manager, contract, response),
        0x20 => respond_to_outburst_story(this, player, club, chairman_weight, stat, response),
        _ => respond_to_other_player_story(this, player, club, contract, age, stat, topic, response),
    }
}

/// Comparer for `apply_player_story_response`: the rehearsal of the copy, then the original, by piece
/// (`player_story_response_piece`).
///
/// The copy runs first with `MEDIA_REACTION_DRY_RUN`: it stops (and the comparison falls back on the original alone)
/// at a call that may post news (the apology, the confidence adjustments, the story effects, the complaint: the whole
/// of the outburst and other-story pieces) or at event flags or contract flags that would make the player request a
/// transfer. Otherwise the regions it wrote are kept, put back, the original runs and they must come out equal: the
/// random generator, `G_CONTRACT_LOOKUP_RESULT`, the club's row in [[`G_MANAGER_MANAGER`]] (0x49 bytes), the player's
/// personality bytes (`.ambition()` .. `.temperament()`), his preferences' favourite and disliked staff (24 bytes), his
/// contract (0x50 bytes) and, for him and the 50 squad slots, the player record's work rate and morale. Inside a
/// recorded tape unit: only the original.
pub(crate) unsafe extern "thiscall" fn compare_apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_APPLY_PLAYER_STORY_RESPONSE);
    if player == 0 || club == 0 || read_usize(G_MANAGER_MANAGER) == 0 || is_recording() {
        return o(this, player, club, topic, response);
    }
    let (p, c) = (Staff(player), Club(club));
    let lookup = read_usize(G_CONTRACT_LOOKUP_RESULT);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.push((manager_manager_row(c), 0x49));
    reg.push((p.addr() + staff::AMBITION, 7));
    let pref = read_usize(p.addr() + staff::STAFF_PREFERENCES);
    if pref != 0 {
        reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 24));
    }
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    if k != 0 {
        reg.push((k, 0x50));
    }
    for s in core::iter::once(player).chain((0..50usize).map(|j| c.squad(j))) {
        let sheet = if s != 0 { Staff(s).player() } else { PlayerSheet(0) };
        if !sheet.is_null() {
            reg.push((sheet.addr() + player_sheet::WORK_RATE, 2));
        }
    }
    write_usize(G_CONTRACT_LOOKUP_RESULT, lookup); // the lookups above do not count
    let before = crate::comparison::MemorySnapshot::read(&reg);
    MEDIA_REACTION_DRY_RUN.store(true, Relaxed);
    MEDIA_REACTION_STOPPED.store(false, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| apply_player_story_response(this, player, club, topic, response)));
    MEDIA_REACTION_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    if MEDIA_REACTION_STOPPED.load(Relaxed) {
        covered!("nma_6 player story only in the original");
        return o(this, player, club, topic, response);
    }
    o(this, player, club, topic, response);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("apply_player_story_response [{}] (player {:#x}, club {:#x}, topic {:#x}, response {}) differences at {:x?}",
            player_story_response_piece(topic), player, club, topic as u16, response as i8, dif)
    });
}

// ---------- batch 294: the action button of a news item (news.cpp) ----------

/// `Friendly::get_tournament(index, date)` (0x5c89a0, friendly.cpp, thiscall, ret 8): the tournament record at an
/// index when it is in use and, with a date, the date matches; otherwise 0.
const FN_FRIENDLY_GET_TOURNAMENT: usize = 0x5c89a0;
/// `read_staff_name_ref_from_news(item, &slot, out)` (0x7e52c0, record_utils.cpp, cdecl): reads 0x18 bytes (four
/// dwords and a date) from the item's slots from `slot` on, moving `slot`.
const FN_READ_STAFF_NAME_REF_FROM_NEWS: usize = 0x7e52c0;
/// `read_match_record_from_news(item, &slot, out)` (0x7e4fa0, record_utils.cpp, cdecl): reads a 0x1f-byte match record
/// (date, competition +8, club index +0x11...) from the item's slots from `slot` on, moving `slot`.
const FN_READ_MATCH_RECORD_FROM_NEWS: usize = 0x7e4fa0;

/// The `cup_tie::FIXTURE_STATE` of a tie whose fixture was created.
const TIE_FIXTURE_CREATED: u8 = 3;

/// Address of data slot `k` of a news item (`news_item::DATA` + 4 × k).
fn action_slot(news: usize, k: usize) -> usize {
    news + news_item::DATA + 4 * k
}

/// Writes the button label (the game's `langlib_sprintf`, so the text goes through the language file) and the screen
/// handler, and returns `r`.
unsafe fn set_action_button(label: usize, handler: *mut u32, text: usize, screen: u32, r: u8) -> u8 {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    sprintf(label, text);
    *handler = screen;
    r
}

/// "Respond" (0xa69488) with the screen handler, needing a reply (2).
unsafe fn respond_button(label: usize, handler: *mut u32, screen: u32) -> u8 {
    set_action_button(label, handler, 0xa69488, screen, 2)
}

/// The piece of `get_action_button` a news item goes to, by the thousands of its type (`get_category`); for the comparer.
fn action_button_piece(kind: i32) -> &'static str {
    match kind {
        1..=999 => "general",
        2000..=2999 => "discipline",
        3000..=3999 => "player",
        4000..=4999 => "transfer",
        5000..=5999 => "reply",
        6000..=0x1790 => "board",
        _ => "none",
    }
}

/// The board news (6000s). Type 0x1774 with slot 1 (byte) 0: "Respond", handler 0x6b2450, 2. Type 0x1786 with slot 1
/// (byte) other than 5, slot 2 (byte) other than 0 and 6 and slot 5 (byte) 0: the "Ultimatum" label (the
/// `<%s - COMMENT (Issue ultimatum to board)>` template 0xa69454 with the text at 0xad9c64), handler 0x9064a0, 1.
/// Anything else: `None`, and the routine goes on to the other categories (which a board item does not match).
unsafe fn board_button(news: usize, label: usize, handler: *mut u32) -> Option<u8> {
    covered!("nma_8 board button");
    let s = |k: usize| action_slot(news, k);
    match read_i32(news) {
        0x1774 => {
            if read_u8(s(1)) == 0 { // nma8: the board request needs a reply
                return Some(respond_button(label, handler, 0x6b2450));
            }
        }
        0x1786 => {
            let (a, b, c) = (read_u8(s(1)), read_u8(s(2)), read_u8(s(5)));
            if a != 5 && b != 0 && b != 6 && c == 0 {
                let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                sprintf(label, 0xa69454usize, 0xad9c64usize); // "Ultimatum<%s - COMMENT (Issue ultimatum to board)>"
                *handler = 0x9064a0;
                return Some(1);
            }
        }
        _ => {}
    }
    None
}

/// The general news (1..999): friendly and tournament invitations, cup draws, national squads, awards.
///
/// - 6, the friendly invitation (slots 0 and 1 the clubs, 2 the club asked, 3 and 4 day and year, 5 the state): the
///   state 3 or 4 gives 0. The club of the manager is club 1 when club 2 is club 0, otherwise club 0; it must be his
///   club job or his national job. The friendly of club 2 on the date (`exe::friendly_find_friendly_match`) and
///   `friendly_check_match_invitation(club 0, club 1, his club, day, year, friendly, no text)` other than 0: "Respond",
///   handler 0x4740c0, 2. The original passes day and year as dwords with the stack's upper halves; the routine does
///   not read them without a text.
/// - 9, the tournament invitation (slot 1 the club, 2 the tournament index, 3 the state, 5 and 6 day and year): state 3
///   or 4 gives 0; the tournament (`Friendly::get_tournament`) and `friendly_check_tournament_invitation(club,
///   tournament, no text)` other than 0: "Respond", handler 0x474720, 2.
/// - 0x16: the manager's club with the id in slot 1, slot 2 (word) equal to the word +0x439 of the object at
///   `G_UEFA_SEEDING` and slot 0 = 0: "Respond", 0x49cc60, 2.
/// - 0x18: his club with the id in slot 0 and slot 1 = 0: "Respond", 0x475070, 2. 0x19: the same with slot 3 = 0:
///   "Respond", 0x4752b0, 2. 0x1e: slot 2 = 0: "Respond", 0x7798c0, 2.
/// - 0x22, the national squad (slot 4 the club, 5 the squad slot, 7 the answer): slot 7 other than 0 gives 0. A news
///   item not of today (`news_item::DATE` against `G_CURRENT_DATE`) gets slot 7 = 1 (`set_item_slot` in the human's
///   inbox and the item) and 0. Otherwise the person mark (`read_person_mark` of `G_NATIONAL_TEAMS`) of the club's
///   nation for `exe::national_teams_get_squad_slot(club, slot 5)`, other than 0: "Respond", 0x77a140, 2.
/// - 0x27, the cup draw (slot 0 the competition, 1 the stage label, 4 the stage index or 0xff, 5 the round, 6 the
///   answer): slot 6 other than 0, no competition (`G_COMP_OBJECT_TABLE`), or a stage index above
///   `cup::CURRENT_STAGE` (signed): 0. The cup (the competition with 0xff, otherwise its stage) must be a cup
///   (`cup::COMP_TYPE` 1) with the stage label of slot 1. When the round is above `cup::CURRENT_ROUND`, the last tie of
///   the round before must have its fixture created. A tie of the round without its fixture: "View Draw", 0x49eac0, 1;
///   all created: slot 6 = 1 (`set_item_slot`) and 0.
/// - 0x28: the FA Cup (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) with `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 1 and slot 0 =
///   1: "Respond", 0x5708f0, 2.
/// - 0x1c, the award nominations: the award (slot 0) and its item (slot 1) in the `G_AWARD_MANAGER` tables with a
///   nominee list (+0x1f) whose +4 word is not 0: "Nominations", 0x416cb0, 1.
/// - anything else: 0.
unsafe fn general_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 general button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    match read_i32(news) {
        6 => {
            let (a, b, c) = (slot(0) as i32, slot(1) as i32, slot(2) as i32);
            let (day, year) = (read_u16(s(3)), read_u16(s(4)));
            let state = read_u8(s(5));
            if state == 3 || state == 4 { // nma8: the friendly invitation is closed
                return 0;
            }
            let mine = if c == a { club_at(b as isize) } else { club_at(a as isize) };
            if me.club_job() != mine && me.national_job() != mine {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let found = exe::friendly_find_friendly_match(friendly, date.as_mut_ptr() as usize, club_at(c as isize));
            if friendly_check_match_invitation(friendly, club_at(a as isize).addr(), club_at(b as isize).addr(), mine.addr(), day as u32, year as u32, found, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x4740c0)
        }
        9 => {
            let state = read_u8(s(3));
            let index = read_u8(s(2));
            let club = club_at(slot(1) as i32 as isize);
            let (day, year) = (read_u16(s(5)), read_u16(s(6)));
            if state == 3 || state == 4 {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let get_tournament: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_FRIENDLY_GET_TOURNAMENT);
            let tournament = get_tournament(friendly, index as u32, date.as_mut_ptr() as usize);
            if friendly_check_tournament_invitation(friendly, club.addr(), tournament, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x474720)
        }
        0x16 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(1) || read_u16(s(2)) != read_u16(read_usize(G_UEFA_SEEDING) + 0x439) || slot(0) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x49cc60)
        }
        0x18 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(1) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x475070)
        }
        0x19 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(3) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4752b0)
        }
        0x1e => {
            if slot(2) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x7798c0)
        }
        0x22 => {
            if slot(7) != 0 {
                return 0;
            }
            if read_u16(G_CURRENT_DATE_YEAR) != read_u16(news + news_item::DATE + 2) || read_u16(G_CURRENT_DATE) != read_u16(news + news_item::DATE) {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, 1, news);
                return 0;
            }
            let club = club_at(slot(4) as i32 as isize);
            let k = exe::national_teams_get_squad_slot(club, slot(5));
            if read_person_mark(read_usize(G_NATIONAL_TEAMS), read_usize(club.addr() + club::NATION), k as i8) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x77a140)
        }
        0x27 => {
            let stage_label = read_u16(s(1));
            let stage = read_u8(s(4));
            let round = read_i16(s(5));
            if slot(6) != 0 {
                return 0;
            }
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            if comp == 0 || (stage as i8 as i32) > read_i32(comp + cup::CURRENT_STAGE) {
                return 0;
            }
            let cup = if stage == 0xff { comp } else { read_usize(read_usize(comp + cup::STAGES).wrapping_add((stage as i8 as isize as usize).wrapping_mul(4))) };
            if read_u8(cup + cup::COMP_TYPE) != 1 || read_u16(cup + cup::STAGE_LABEL) != stage_label {
                return 0;
            }
            let r = round as i32 as usize;
            if (read_u8(cup + cup::CURRENT_ROUND) as i8 as i16) < round {
                // the last tie of the round before must have its fixture
                let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_sub(1).wrapping_mul(0x68)) + 0x1a) as i32 as usize;
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_sub(1).wrapping_mul(4)));
                if read_u8(tie_table.wrapping_add(ties.wrapping_sub(1).wrapping_mul(0x35)) + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                    return 0;
                }
            }
            let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_mul(0x68)) + 0x1a);
            if ties > 0 {
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_mul(4)));
                for k in 0..ties {
                    if read_u8(tie_table + k as usize * 0x35 + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                        return set_action_button(label, handler, 0xa69448, 0x49eac0, 1); // "View Draw"
                    }
                }
            }
            set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 6, 1, news);
            0
        }
        0x28 => {
            let fa_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_ENGLISH_FA_CUP).wrapping_mul(4)));
            if read_u8(fa_cup + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 1 || slot(0) != 1 {
                return 0;
            }
            respond_button(label, handler, 0x5708f0)
        }
        0x1c => {
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            let item = read_usize(read_usize(award + 4).wrapping_add((slot(1) as usize).wrapping_mul(4)));
            let nominees = read_usize(item + 0x1f);
            if nominees == 0 || read_u16(nominees + 4) == 0 {
                return 0;
            }
            set_action_button(label, handler, 0x987ff8, 0x416cb0, 1) // "Nominations"
        }
        _ => 0,
    }
}

/// The player news (3000s): contracts, media stories, bids for a player.
///
/// - 0xbbf: the manager's club job with the id in slot 0 and slot 7 (byte) 0: "Respond", 0x4dca40, 2.
/// - 0xbe8: the club of slot 8 managed by him (`club::MANAGER`), slot 18 (byte) 0 and slot 15 = 0: "Respond",
///   0x741f50, 2. 0xbe6: the club of slot 9 managed by him and slot 20 (byte) 0: the same.
/// - 0xbd9: slot 8 (byte) 0, then as 0xbd0. 0xbd0: the club of slot 4 managed by him and slot 10 (byte) 0: "Respond",
///   0x741f50, 2. 0xbdd: the same with slot 13 (byte). 0xbdf: the same with slot 10 (byte), except for a story kind
///   (slot 9, signed byte) of 0x3e, 0x50, 0x51 or 0x5e, which gives 0 (the original's jump table).
/// - 0xbc1: his club job with the id in slot 0 and slot 6 (byte) 0: "Respond", 0x8f66d0, 2. 0xbc2: the same with slot
///   7 (byte): "Offer Contract", 0x4dc9b0, 1.
/// - 0xbb8, the contract talks of a player (slot 0 the person, 4 the club, 8 the transfer offer or −1, 9 its serial,
///   10 the state): his club job with the id in slot 4; with an offer, the offer (`exe::transfer_manager_get_offer_slot`)
///   with the serial of slot 9; without one, slot 10 other than 5; then the contract offer of the person at the club
///   (`exe::contract_manager_find_contract_offer`) with `contract_offer::STATE` 3: "Negotiate", 0x4e5770, 2.
/// - anything else: 0.
unsafe fn player_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 player button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    let managed_by_me = |k: usize| club_at(slot(k) as i32 as isize).manager().addr() == manager;
    let own_club_is = |id: u32| !me.club_job().is_null() && read_u32(me.club_job().addr()) == id;
    let story = |club_slot: usize, flag_slot_addr: usize| {
        if !managed_by_me(club_slot) || read_u8(flag_slot_addr) != 0 {
            return 0;
        }
        respond_button(label, handler, 0x741f50)
    };
    match read_i32(news) {
        0xbbf => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4dca40)
        }
        0xbe8 => {
            if !managed_by_me(8) || read_u8(s(18)) != 0 || slot(15) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x741f50)
        }
        0xbe6 => story(9, s(20)),
        0xbd9 => {
            if read_u8(s(8)) != 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdf => {
            // the original's jump table on (story kind - 0x3e), 0..0x20: 0 ends the routine
            const KINDS: [u8; 33] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];
            let k = (read_u8(s(9)) as i8 as i32).wrapping_sub(0x3e) as u32;
            if k <= 0x20 && KINDS[k as usize] == 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdd => story(4, s(13)),
        0xbd0 => story(4, s(10)),
        0xbc1 => {
            if !own_club_is(slot(0)) || read_u8(s(6)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x8f66d0)
        }
        0xbc2 => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            set_action_button(label, handler, 0x9aff10, 0x4dc9b0, 1) // "Offer Contract"
        }
        0xbb8 => {
            let (person, club, offer, serial, state) = (slot(0), slot(4), slot(8) as i32, slot(9), read_u8(s(10)));
            if !own_club_is(club) {
                return 0;
            }
            if offer != -1 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer)));
                if o.is_null() || o.serial() as u32 != serial {
                    return 0;
                }
            } else if state == 5 {
                return 0;
            }
            let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), person, club);
            if k == 0 || read_u8(k + contract_offer::STATE) != 3 { // nma8: the player's contract talks need a reply
                return 0;
            }
            set_action_button(label, handler, 0xa6943c, 0x4e5770, 2) // "Negotiate"
        }
        _ => 0,
    }
}

/// The discipline news (2000s): types 0x7d2 and 0x7d3, the ban that can be appealed (slot 0 the appeal made, 4 and 5
/// day and year of the ban, 10 the kind, 12 the appeal pending, 15 and 16 day and year of the hearing, 23 a match).
/// The person (`read_staff_name_ref_from_news` from slot 17) must be a valid id. With a match (slot 23), the match
/// record from slot 24 must have a valid club and competition (`G_CLUB_COMP_TABLE`), the club must not be a national
/// team, the competition must allow appeals (+0x5d) and the person must be at the manager's club. The appeal needs
/// the club's nation not to be Argentina (`club_nation_is_not_argentina`) and a kind other than 0, 1 and 4, no appeal
/// made or pending, and the hearing today or the ban still to start (its year above today's, or the same year and a
/// later day): "Appeal", 0x55e620, 1. Otherwise 0.
unsafe fn discipline_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 discipline button");
    let kind = read_i32(news);
    if kind != 0x7d3 && kind != 0x7d2 {
        return 0;
    }
    let s = |k: usize| action_slot(news, k);
    let appealed = read_u8(s(0));
    let (ban_day, ban_year) = (read_u16(s(4)), read_u16(s(5)));
    let (hearing_day, hearing_year) = (read_u16(s(15)), read_u16(s(16)));
    let ban_kind = read_u32(s(10));
    let pending = read_u32(s(12));
    let read_name: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_READ_STAFF_NAME_REF_FROM_NEWS);
    let read_match: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_READ_MATCH_RECORD_FROM_NEWS);
    let mut next = 0x11u8;
    let mut person = [0u8; 0x18];
    read_name(news, &mut next, person.as_mut_ptr());
    let id = read_i32(person.as_ptr() as usize + 0xc);
    if id < 0 || id > read_i32(G_STAFF_COUNT) - 1 {
        return 0;
    }
    if read_u32(s(23)) != 0 {
        next = 0x18;
        let mut game = [0u8; 0x20];
        read_match(news, &mut next, game.as_mut_ptr());
        let club_index = read_i32(game.as_ptr() as usize + 0x11);
        if club_index < 0 || club_index > read_i32(G_CLUB_COUNT) - 1 {
            return 0;
        }
        let comp_index = read_i32(game.as_ptr() as usize + 8);
        if comp_index < 0 || comp_index > read_i32(G_CLUB_COMP_COUNT) - 1 {
            return 0;
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_index as usize).wrapping_mul(0x6b));
        if exe::database_is_national_team(club_at(club_index as isize)) != 0 || read_u32(comp + 0x5d) == 0
            || Staff(manager).club_job() != staff_at(id as isize).club_job() {
            return 0;
        }
    }
    let mut can = club_nation_is_not_argentina(Staff(manager).club_job().addr()) as u8;
    if ban_kind == 4 || ban_kind == 0 || ban_kind == 1 {
        can = 0;
    }
    if appealed != 0 || can == 0 || pending != 0 {
        return 0;
    }
    let (year, day) = (read_u16(G_CURRENT_DATE_YEAR), read_u16(G_CURRENT_DATE));
    let hearing_today = hearing_year == year && hearing_day == day;
    if !hearing_today && !(ban_year > year || (ban_year == year && (ban_day as i16) > (day as i16))) {
        return 0;
    }
    set_action_button(label, handler, 0xa69434, 0x55e620, 1) // "Appeal"
}

/// The transfer news (4000s), on the offers of `G_TRANSFER_MANAGER` (slot 6, 7 or 11 the offer index; the offer must
/// have the serial (`offer::SERIAL`) the news keeps).
///
/// - 0xfa0, a bid for one of his players: the club of slot 5 managed by him, the serial of slot 7, the kind (slot 9,
///   byte) not 1, 2, 5 or 0x11 and the offer in state 0: "Respond", 0x8f1730, 2.
/// - 0xfbc: the club of slot 4 (above −1) managed by him and slot 7 (byte) 0: "Respond", 0x741f50, 2.
/// - 0xfa2 and 0xfc0, a negotiation (slot 9 the negotiation's serial): the offer of slot 6 with the serial of slot 7,
///   a negotiation (`offer::NEGOTIATION_INDEX`, `get_negotiation_slot`) with the serial of slot 9 and the offer in
///   state 1: "Respond", 0x8f1830 (0xfa2) or 0x8f93b0 (0xfc0), 2.
/// - 0xfac, personal terms (slot 0 the person, 7 the offer, 8 its serial, 9 the kind): kind 1 or 4 gives 0; the
///   offer open (`exe::transfer_offer_is_closed` 0) with the serial and not in state 1. Bought by his club
///   (`offer::BUYER_CLUB_ID`): with no terms yet (`offer::TERMS_STATE` 0) or the offer in state 5 or 6, "Offer
///   Contract", 0x8f1950, 2. Otherwise, when the target's contract (`exe::contract_manager_get_contract`, kept in
///   `G_CONTRACT_LOOKUP_RESULT`) is at his club (or both are none), a player of the exchange list
///   (`offer::EXCHANGE_PLAYERS`) with the id of slot 0 and +4 = 0: the same button.
/// - 0xfa4 (slot 6 the step, 7 the offer or −1, 8 its serial, 9 a flag): step 3, an offer with the serial in state 7
///   or 8 and the contract offer of the person (slot 0) at the club of slot 5 in state 3: "Negotiate", 0x4e5770, 2.
///   Step 4 with the flag 0, an offer with the serial, his club job with the id of slot 5, the offer in state 0xf and
///   `offer::RESPONSE_ROUND` below 3: "Negotiate", 0x4e5770, 1. When no button and an offer index other than −1:
///   slot 7 = −1 (`set_item_slot`) and 0.
/// - 0xfa5 (slot 6 the offer, 7 its serial, 9 the kind; kinds 2, 3, 0xc, 0xd, 0xe, 0x10, 0x11, 0x12 give 0): bought by
///   his club and the offer in state 0xb or 0xc; or the target's contract at his club, no pre-contract and the offer
///   in state 0xb or 0xd: "Confirm", 0x8f2d20, 2.
/// - 0xfab: his club job with the id of slot 0 or 1 and slot 7 (byte) 0: "Respond", 0x8f1a60, 2.
/// - 0xfb4, a contract renewal for a bid (slot 0 the person, 5 his club id, 10 the wish, 11 the offer, 12 its serial,
///   13 a flag): the offer's buyer is his club job, the person has a contract at a club with the id of slot 5, the
///   serial, no pre-contract, state 1, the flag 0 and the wish 1: "Re-Negotiate", 0x8f1b40, 2.
/// - 0xfb5: as 0xfa0 without the kind: "Respond", 0x8f1c30, 2.
/// - 0xfc1: "Details", 0x77b700, 1. 0xfa8, 0xfb6 and the rest: 0.
unsafe fn transfer_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 transfer button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let tm = read_usize(G_TRANSFER_MANAGER);
    let offer_slot = |k: usize| exe::transfer_manager_get_offer_slot(tm, slot(k) as i32);
    let managed_by_me = |i: i32| club_at(i as isize).manager().addr() == manager;
    let own = Staff(manager).club_job().addr();
    // the club of a person's contract (`exe::contract_manager_get_contract`, kept in G_CONTRACT_LOOKUP_RESULT), or 0
    let contract_club = |person: u32| {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff_at(person as i32 as isize));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() }
    };
    let negotiated = |k: usize| {
        let os = offer_slot(6);
        let o = Offer(read_usize(os));
        if o.is_null() || o.serial() as u32 != slot(7) || o.negotiation_index() as i32 == -1 {
            return false;
        }
        let n = read_usize(get_negotiation_slot(tm, o.negotiation_index() as i32));
        slot(k) == read_u32(n + offer::SERIAL) && Offer(read_usize(os)).status() == 1
    };
    match read_i32(news) {
        0xfa0 => {
            let o = Offer(read_usize(offer_slot(6)));
            let (club, kind) = (slot(5) as i32, read_u8(s(9)));
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && kind != 1 && kind != 5 && kind != 2 && kind != 0x11
                && o.status() == 0 { // nma8: a bid for his player needs a reply
                return respond_button(label, handler, 0x8f1730);
            }
            0
        }
        0xfbc => {
            let club = slot(4) as i32;
            if club > -1 && managed_by_me(club) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x741f50);
            }
            0
        }
        0xfa2 => if negotiated(9) { respond_button(label, handler, 0x8f1830) } else { 0 },
        0xfc0 => if negotiated(9) { respond_button(label, handler, 0x8f93b0) } else { 0 },
        0xfac => {
            let person = slot(0);
            let kind = read_u8(s(9));
            let os = offer_slot(7);
            if kind == 1 || kind == 4 {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(8) || exe::transfer_offer_is_closed(o) != 0 || o.status() == 1 {
                return 0;
            }
            if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                if o.terms_state() == 0 || o.status() == 5 || o.status() == 6 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
                return 0;
            }
            if contract_club(o.target_staff_id() as u32) != own {
                return 0;
            }
            let list = Llist(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
            if list.is_null() {
                return 0;
            }
            list.set_cursor(list.head());
            loop {
                let p = exe::llist_get_next(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
                if p == 0 {
                    return 0;
                }
                if read_u32(p) == person && read_u8(p + 4) == 0 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
            }
        }
        0xfa4 => {
            let step = read_u8(s(6));
            let index = slot(7) as i32;
            let flag = read_u8(s(9));
            if (step == 3 || step == 4) && index == -1 {
                return 0;
            }
            if step == 3 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && (o.status() == 7 || o.status() == 8) {
                    let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), slot(0), slot(5));
                    if k != 0 && read_u8(k + contract_offer::STATE) == 3 {
                        return set_action_button(label, handler, 0xa6943c, 0x4e5770, 2); // "Negotiate"
                    }
                }
            } else if step == 4 && flag == 0 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && own != 0 && read_u32(own) == slot(5) && o.status() == 0xf && o.response_round() < 3 {
                    return set_action_button(label, handler, 0xa6943c, 0x4e5770, 1); // "Negotiate"
                }
            }
            if index != -1 {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, -1i32 as u32, news);
            }
            0
        }
        0xfa5 => {
            let kind = read_u8(s(9));
            if [0xe, 0xd, 0xc, 3, 2, 0x10, 0x12, 0x11].contains(&kind) {
                return 0;
            }
            let os = offer_slot(6);
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(7) {
                return 0;
            }
            let confirm = if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                o.status() == 0xb || o.status() == 0xc
            } else {
                let o2 = Offer(read_usize(os));
                contract_club(o.target_staff_id() as u32) == own && o2.pre_contract() == 0 && (o2.status() == 0xb || o2.status() == 0xd)
            };
            if confirm {
                return set_action_button(label, handler, 0x98a198, 0x8f2d20, 2); // "Confirm"
            }
            0
        }
        0xfab => {
            if own != 0 && (read_u32(own) == slot(0) || read_u32(own) == slot(1)) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x8f1a60);
            }
            0
        }
        0xfb4 => {
            let (wish, serial, flag, person, club_id) = (read_u8(s(10)), slot(12), read_u8(s(13)), slot(0), slot(5));
            let os = exe::transfer_manager_get_offer_slot(tm, slot(11) as i32);
            if os == 0 || read_usize(os) == 0 {
                return 0;
            }
            if club_at(Offer(read_usize(os)).buyer_club_id() as i32 as isize).addr() != own {
                return 0;
            }
            if contract_club(person) == 0 {
                return 0;
            }
            if read_u32(contract_club(person)) != club_id {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.serial() as u32 == serial && o.pre_contract() == 0 && o.status() == 1 && flag == 0 && wish == 1 {
                return set_action_button(label, handler, 0xa69424, 0x8f1b40, 2); // "Re-Negotiate"
            }
            0
        }
        0xfb5 => {
            let o = Offer(read_usize(offer_slot(6)));
            let club = slot(5) as i32;
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && o.status() == 0 {
                return respond_button(label, handler, 0x8f1c30);
            }
            0
        }
        0xfc1 => set_action_button(label, handler, 0xa6941c, 0x77b700, 1), // "Details"
        _ => 0,
    }
}

/// Writes the button label and the screen handler of a news item for a manager and says whether it needs a reply.
///
/// `NewsItem::get_action_button(manager, label, size, &handler)`: no manager (box of line 0x123b), no label or a size
/// below 0x33 (line 0x1243) or no handler (line 0x124e, after the label is emptied): the "Error" box and 0. Otherwise
/// the label is emptied, the handler set to 0 and the item goes to the piece of its category (`get_category`, asked
/// again before each test, as the original does): 6 the board (`board_button`), 2 the general news
/// (`general_button`), 4 the player news (`player_button`), 3 the discipline (`discipline_button`), 5 the transfers
/// (`transfer_button`) and 1, type 0x1388 only, "Reply" with handler 0x789860 and 1. A board item that gets no button
/// falls through the other tests to 0. Returns 2 when the item needs a reply (the automatic reply
/// `NewsItem::auto_respond` 0x784100, `mark_item_read` and `has_urgent_items` look for it), 1 for a button without a
/// needed reply, 0 for none. Some items are also closed on the way: the national squad (0x22) not of today and the
/// cup draw (0x27) with every fixture made get their answer slot set, and the transfer step (0xfa4) without a button
/// gets its offer slot set to −1 (`set_item_slot`).
/// Original: `NewsItem::get_action_button` (0x782b70, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    if manager == 0 {
        show_error_at(0xa693ec, 0x123b);
        return 0;
    }
    if label == 0 || (size as i32) < 0x33 {
        show_error_at(0xa693ec, 0x1243);
        return 0;
    }
    write_u8(label, 0);
    if handler.is_null() {
        show_error_at(0xa693ec, 0x124e);
        return 0;
    }
    *handler = 0;
    let category = || get_category(news) as u8;
    if category() == 6 {
        if let Some(r) = board_button(news, label, handler) {
            return r;
        }
    }
    if category() == 2 {
        return general_button(news, manager, label, handler);
    }
    if category() == 4 {
        return player_button(news, manager, label, handler);
    }
    if category() == 3 {
        return discipline_button(news, manager, label, handler);
    }
    if category() == 5 {
        return transfer_button(news, manager, label, handler);
    }
    if category() == 1 && read_i32(news) == 0x1388 {
        covered!("nma_8 reply button");
        return set_action_button(label, handler, 0xa69414, 0x789860, 1); // "Reply"
    }
    0
}

/// Comparer for `get_action_button`: the original and the copy side by side (the label, the handler, the contract
/// lookup global, the item and the news storage that `set_item_slot` writes); a divergence names the piece (the
/// item's category). The error boxes (no manager, label or handler, a type outside the categories) run the original
/// only.
pub(crate) unsafe extern "thiscall" fn compare_get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8>(IDX_GET_ACTION_BUTTON);
    let kind = read_i32(news);
    if manager == 0 || label == 0 || (size as i32) < 0x33 || handler.is_null() || !(1..=0x1790).contains(&kind) {
        return o(news, manager, label, size, handler);
    }
    let store = read_usize(G_NEWS);
    let mut regions = vec![
        (label, (size as usize).min(0x7d0)),
        (handler as usize, 4usize),
        (G_CONTRACT_LOOKUP_RESULT, 4),
        (news, NEWS_NEWS_ITEM_SIZE),
    ];
    if store != 0 && read_usize(store) != 0 {
        regions.push((read_usize(store), read_i32(store + news_fields::SLOT_COUNT).max(0) as usize * NEWS_NEWS_ITEM_SIZE));
    }
    let (r, n, same, _, _) = crate::comparison::side_by_side(&regions,
        || o(news, manager, label, size, handler),
        || get_action_button(news, manager, label, size, handler));
    SCOREBOARD.record(same && r == n, || format!("get_action_button piece {} (type {:#x}) original={} rust={} handler {:#x} label {:?} memory equal {}",
        action_button_piece(kind), kind, r, n, *handler, (0..0x40).map(|k| read_u8(label + k)).take_while(|&b| b != 0).map(|b| b as char).collect::<String>(), same));
    r
}
