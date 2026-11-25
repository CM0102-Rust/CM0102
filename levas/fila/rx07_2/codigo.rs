// ---------- rx07_2: the staff contracts built for a new game (contract_manager.cpp) ----------

/// Whether a person's job at his club (`staff::JOB_FOR_CLUB`) takes a contract when the game is created: 0, 6 to 13
/// and 15 (not 1 to 5 nor 14).
pub(crate) fn job_takes_initial_contract(job: u8) -> bool {
    matches!(job, 0 | 6..=13 | 15)
}

/// The unset date of the database (day 31 of 1900), checked year first, as the original compares with
/// `Date::Date(0x1f, 0x76c)`.
pub(crate) unsafe fn is_unset_database_date(date: usize) -> bool {
    read_u16(date + 2) == 0x76c && read_i16(date) == 0x1f
}

/// The club contract end of person `s` at the start of a new game: an unset one (31/1900) gets day 1 to 28 of the
/// month of the club's season date (`exe::key_nation_club_in_season`) plus 10 days, `1 + utils_random(6)` years after
/// the current year; then any end not past the game start plus 8 weeks moves on by 52 weeks until it is.
pub(crate) unsafe fn set_initial_club_contract_end(s: Staff) {
    let expires = s.addr() + staff::DATE_EXPIRES_CLUB;
    let mut scratch = [0u8; 8];
    if is_unset_database_date(expires) {
        let year = read_u16(G_CURRENT_DATE + 2) as u32;
        let mut season = [0u32; 2];
        exe::date_date_54a110(season.as_mut_ptr() as usize);
        exe::key_nation_club_in_season(s.club_job(), 0, season.as_mut_ptr() as usize);
        let mut later = [0u32; 2];
        let month = exe::date_get_month(exe::date_add_days(season.as_ptr() as usize, later.as_mut_ptr() as usize, 10)) as u32;
        let years = exe::utils_random(6) as u32;
        let day = (exe::utils_random(0x1c) as u8).wrapping_add(1) as i8 as i16 as u32;
        let mut end = [0u16; 4];
        let d = exe::date_date_549f70(&mut end, day, month, years.wrapping_add(year).wrapping_add(1), u32::MAX);
        exe::date_operator_3d(expires, scratch.as_mut_ptr() as usize, d);
    }
    loop {
        let mut limit = [0u8; 8];
        let t = exe::date_add_weeks(G_GAME_START_DATE, &mut limit, 8);
        let (ty, ey) = (read_u16(t + 2), read_u16(expires + 2));
        if !(ty > ey || (ty == ey && read_i16(t) > read_i16(expires))) {
            break;
        }
        covered!("rx07_2 contract end moved a year");
        let mut next = [0u8; 8];
        let n = exe::date_add_weeks(expires, &mut next, 0x34);
        exe::date_operator_3d(expires, scratch.as_mut_ptr() as usize, n);
    }
}

/// The national contract end of person `s` at the start of a new game: an unset one (31/1900) becomes 10 June,
/// `1 + utils_random(5)` years after the current year.
pub(crate) unsafe fn set_initial_national_contract_end(s: Staff) {
    let expires = s.addr() + 0x31; // the national contract end (TCMDate after staff::DATE_JOINED_NATION)
    if !is_unset_database_date(expires) {
        return;
    }
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let years = exe::utils_random(5) as u32;
    let mut end = [0u16; 4];
    let d = exe::date_date_549f70(&mut end, 10, 5, years.wrapping_add(year).wrapping_add(1), u32::MAX);
    let mut scratch = [0u8; 8];
    exe::date_operator_3d(expires, scratch.as_mut_ptr() as usize, d);
}

/// Rule: the staff contracts of a new game. The manager gets an 80-byte contract per person (`G_STAFF_COUNT`, kept at
/// +4; none left: the "Memory Allocation Error" box at line 0xd7 and nothing else) and no contracts in use (+8). For
/// each person, in table order: bit 7 of the staff state record's flags (`staff_state::STATUS_FLAGS`) is cleared; his
/// value (`staff::VALUE`) is worked out (`exe::contract_manager_calc_player_value` with his club, 1, -1, 0); with a
/// club, the club contract end is set (`set_initial_club_contract_end`); with a national team, the national one
/// (`set_initial_national_contract_end`); with a club and a job that takes a contract (`job_takes_initial_contract`), a
/// free contract (`exe::contract_manager_get_free_staff_contract_space`) becomes his main one and is generated
/// (`exe::staff_contract_generate_initial_contract` with his value) and, for a player, the squad number of his player
/// sheet (clamped to 1..50) is kept negated in the contract.
///
/// The original builds a STAFF_CONTRACT on its stack (0x85e410) that nothing reads; the copy leaves it out.
/// Defect of the original: with no contract given to a player (no free space), the squad number goes to address 0x3a
/// (the null contract); the copy does the same.
/// Original: `CONTRACT_MANAGER::initialise_all_staff_contracts` (0x4c7560, contract_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn initialise_all_staff_contracts(this: usize) {
    let count = read_i32(G_STAFF_COUNT);
    write_i32(this + 4, count); // the contract capacity
    let contracts = exe::crt_malloc((count as usize).wrapping_mul(80) as u32);
    write_usize(this, contracts);
    if contracts == 0 {
        award_load_error_box(0x9a8190, 0xd7, 0x9a8134, (0x9a8108, 0x9a7ff8));
        return;
    }
    write_u32(this + 8, 0); // contracts in use
    for id in 0..count.max(0) as usize {
        let state = contract_manager_staff_state_record(id);
        write_u8(state + staff_state::STATUS_FLAGS, read_u8(state + staff_state::STATUS_FLAGS) & 0x7f);
        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE)));
        s.set_value(exe::contract_manager_calc_player_value(this, s, s.club_job(), 1, -1, 0) as usize);
        if !s.club_job().is_null() {
            set_initial_club_contract_end(s);
        }
        if !s.national_job().is_null() {
            set_initial_national_contract_end(s);
        }
        if s.club_job().is_null() || !job_takes_initial_contract(s.job_for_club()) {
            continue;
        }
        covered!("rx07_2 initial contract");
        let value = s.value();
        let k = exe::contract_manager_get_free_staff_contract_space(this, 1);
        if k >= 0 {
            let person = read_usize(s.addr());
            write_i32(contract_manager_staff_state_record(person), k);
            let c = (k as usize).wrapping_mul(80).wrapping_add(read_usize(this));
            write_usize(c, person);
            exe::staff_contract_generate_initial_contract(c, s, value as u32);
        }
        let sheet = s.player();
        if sheet.is_null() {
            continue;
        }
        let number = read_i8(sheet.addr() + 4).clamp(1, 50); // the player sheet's squad number
        let person = read_i32(s.addr());
        let mut c = 0usize;
        if person < read_i32(G_STAFF_COUNT) {
            let i = read_i32(contract_manager_staff_state_record(person as usize));
            if i >= 0 {
                c = (i as usize).wrapping_mul(80).wrapping_add(read_usize(this));
            }
        }
        write_i8(c + contract::SQUAD_NUMBER, number.wrapping_neg());
    }
}

/// Comparer for `initialise_all_staff_contracts`: only the original (it allocates and draws the contract ends); then
/// every person whose club job takes a contract must have a main contract that names him, and the contract count must
/// not pass the capacity.
pub(crate) unsafe extern "thiscall" fn compare_initialise_all_staff_contracts(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    if read_usize(this) == 0 {
        return;
    }
    let mut wrong = Vec::new();
    for id in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE)));
        if s.club_job().is_null() || !job_takes_initial_contract(s.job_for_club()) {
            continue;
        }
        let i = read_i32(contract_manager_staff_state_record(id));
        if i < 0 || read_usize((i as usize).wrapping_mul(80).wrapping_add(read_usize(this))) != id {
            wrong.push(id);
        }
    }
    let (n, cap) = (read_i32(this + 8), read_i32(this + 4));
    SCOREBOARD.record(wrong.is_empty() && n <= cap, || format!("initialise_all_staff_contracts: {} contracts of {}, people without one {:?}", n, cap, &wrong[..wrong.len().min(20)]));
}
