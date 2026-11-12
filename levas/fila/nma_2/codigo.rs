// ---------- batch %%LEVA%%: the season update pass (game.cpp) ----------

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
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}
