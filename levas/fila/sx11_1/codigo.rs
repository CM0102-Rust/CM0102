// ---------- batch %%LEVA%%: the run-up to the start date (game.cpp) ----------

/// Exe routines the run-up to the start date calls without an `exe::` wrapper (cm0102.exe 3.9.68; most are Rust and
/// run through their detour). Conventions checked at the call sites in 0x5cc850.
#[allow(dead_code)]
pub(crate) mod start_date_fns {
    pub(crate) const SET_UP_THE_DAY: usize = 0x6b3c10; // MATCH_DAY::set_up_the_day thiscall(day, date, slice) ret 8
    pub(crate) const PLAY_THE_DAY: usize = 0x6b4360; // MATCH_DAY::play_the_day thiscall(day)
    pub(crate) const END_DAY: usize = 0x6b4190; // MATCH_DAY::end_day thiscall(day)
    pub(crate) const STAFF_RECORDS_DAILY_UPDATE: usize = 0x86dda0; // thiscall(this, date) ret 4
    pub(crate) const CONTRACT_MANAGER_DAILY_UPDATE: usize = 0x4c7ac0; // thiscall(this, date, slice, people) ret 0xc
    pub(crate) const UPDATE_CITY_WEATHER: usize = 0x922ee0; // Weather thiscall(this)
    pub(crate) const DISCIPLINE_DAILY_UPDATE: usize = 0x556240; // thiscall(this, date) ret 4
    pub(crate) const INJURY_DAILY_UPDATE: usize = 0x62b7b0; // thiscall(this)
    pub(crate) const AWARD_MANAGER_PROCESS_DATE: usize = 0x413540; // thiscall(this, date) ret 4
    pub(crate) const HOST_COUNTRY_DAILY_UPDATE: usize = 0x5fa850; // thiscall(this, date) ret 4
    pub(crate) const NATIONAL_TEAMS_DAILY_UPDATE: usize = 0x76b100; // thiscall(this, date) ret 4
    pub(crate) const FINANCE_TABLE_DAILY_UPDATE: usize = 0x59b9c0; // thiscall(this, date) ret 4
    pub(crate) const OFFICIALS_CLEAR_BUSY_FLAGS: usize = 0x79bf40; // thiscall(this)
    pub(crate) const MANAGER_MANAGER_DAILY_UPDATE: usize = 0x68f9c0; // thiscall(this)
    pub(crate) const STADIUM_DAILY_UPDATE: usize = 0x85b820; // thiscall(this, date) ret 4
    pub(crate) const CLUB_RECORDS_SEND_RECORD_NEWS: usize = 0x448e30; // thiscall(this)
    pub(crate) const ASSESS_CLUB_SQUADS: usize = 0x83a1b0; // Shortlist_Manager thiscall(this) ("Assessing Club Squads")
    pub(crate) const INJURY_INIT_SQUAD_FITNESS: usize = 0x62a920; // thiscall(this)
    pub(crate) const GAME_SEASON_UPDATE_PASS: usize = 0x5d5ae0; // cdecl()
    pub(crate) const GAME_UPDATE_NATION_COEFFICIENTS: usize = 0x5d5f20; // cdecl()
    /// The game phase byte saved with the game (+ field of the save at 0xae2cfc): 2 once the run-up is over [inference].
    pub(crate) const G_GAME_PHASE: usize = 0xae2cfc;
    /// The progress bar's status text buffer (the "Initialising Leagues" text).
    pub(crate) const G_STATUS_TEXT: usize = 0xde1f64;
    pub(crate) const TEXT_INITIALISING_LEAGUES: usize = 0x9cd244; // "Initialising Leagues"
}

/// `this` routine of the exe called with no argument (thiscall).
unsafe fn start_date_call0(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(this)
}

/// `this` routine of the exe called with the game date (thiscall, ret 4).
unsafe fn start_date_call_today(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    g(this, G_CURRENT_DATE)
}

/// Whether the game date is still before the start date `start` (day i16, year u16): the year unsigned first, then the
/// day of the year signed.
unsafe fn before_start_date(start: &[u16; 4]) -> bool {
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if start[1] != year {
        return start[1] > year;
    }
    start[0] as i16 > read_i16(G_CURRENT_DATE)
}

/// The first part of a day slice: the match day of the slice (built, played, ended and freed; `G_MATCH_DAY` points at
/// it meanwhile), the fixtures, the staff records, the competitions (`game_update_comps_for_date(0)`) and the
/// contracts.
unsafe fn start_date_slice_matches(slice: u8) {
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
unsafe fn start_date_slice_world(counter: &mut i32, total: i32) {
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
unsafe fn start_date_next_day() {
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
    original_fn::<unsafe extern "C" fn()>(%%I0%%)();
    let mut out = [0u16; 4];
    let yesterday = exe::date_sub_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    let last_ok = read_u16(G_LAST_PROCESSED_DATE) == read_u16(yesterday) && read_u16(G_LAST_PROCESSED_DATE + 2) == read_u16(yesterday + 2);
    let date_ok = !ran || (read_u16(G_CURRENT_DATE) == start[0] && read_u16(G_CURRENT_DATE_YEAR) == start[1] && read_u8(G_CURRENT_DAY_SLICE) == 0);
    let ok = last_ok && date_ok && read_usize(G_MATCH_DAY) == 0 && read_u8(start_date_fns::G_GAME_PHASE) == 2;
    SCOREBOARD.record(ok, || format!("game_run_to_start_date: ran {} date {}/{} slice {} (start {}/{}), last processed {}/{}, match day {:x}, phase {}",
        ran, read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR), read_u8(G_CURRENT_DAY_SLICE), start[0] as i16, start[1],
        read_i16(G_LAST_PROCESSED_DATE), read_u16(G_LAST_PROCESSED_DATE + 2), read_usize(G_MATCH_DAY), read_u8(start_date_fns::G_GAME_PHASE)));
}
