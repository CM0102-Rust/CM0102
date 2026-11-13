// ---------- batch %%LEVA%%: the hall of fame entries of the human managers (hall_of_fame.cpp) ----------

/// Constants of the hall of fame (`G_HALL_OF_FAME`: 28 tables of 100 entries of 120 bytes from +0).
pub(crate) mod hall_of_fame_consts {
    /// One table: 100 entries of `ENTRY_SIZE` bytes, kept sorted (best first).
    pub(crate) const TABLE_SIZE: usize = 12000;
    pub(crate) const TABLES: usize = 28;
    pub(crate) const ENTRIES: usize = 100;
    /// An entry: +0 manager name (50 bytes), +0x32 club/nation (50 bytes), +0x64 time stamp, +0x68 score, +0x6c the
    /// game's random seed (`G_GAME_RANDOM_SEED`), +0x70 date.
    pub(crate) const ENTRY_SIZE: usize = 120;
    pub(crate) const NAME_LEN: usize = 50;
    pub(crate) const CLUB: usize = 0x32;
    pub(crate) const TIME: usize = 0x64;
    pub(crate) const SCORE: usize = 0x68;
    pub(crate) const SEED: usize = 0x6c;
    pub(crate) const DATE: usize = 0x70;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\hall_of_fame.cpp" (the error box), "%s/%s" and "No Club/Nation".
    pub(crate) const HALL_OF_FAME_CPP: usize = 0x9ce408;
    pub(crate) const CLUB_AND_NATION: usize = 0x9ce448;
    pub(crate) const NO_CLUB_OR_NATION: usize = 0x9ce438;
    /// CRT strncpy (0x9466d0), time (0x946ebb) and the entry order (score high to low, then older first).
    pub(crate) const STRNCPY: usize = 0x9466d0;
    pub(crate) const TIME_FN: usize = 0x946ebb;
    pub(crate) const COMPARE_ENTRIES: usize = 0x5f0620;
    /// The league tables 1..27, in order, by the nation key of their league (`KEY_NATION_*`); table 0 is the calendar
    /// year.
    pub(crate) const LEAGUE_NATIONS: [usize; 27] = [
        super::KEY_NATION_ARGENTINA, super::KEY_NATION_AUSTRALIA, super::KEY_NATION_BELGIUM, super::KEY_NATION_BRAZIL,
        super::KEY_NATION_CROATIA, super::KEY_NATION_DENMARK, super::KEY_NATION_ENGLAND, super::KEY_NATION_FINLAND,
        super::KEY_NATION_FRANCE, super::KEY_NATION_GERMANY, super::KEY_NATION_GREECE, super::KEY_NATION_HOLLAND,
        super::KEY_NATION_REPUBLIC_OF_IRELAND, super::KEY_NATION_ITALY, super::KEY_NATION_JAPAN,
        super::KEY_NATION_NORTHERN_IRELAND, super::KEY_NATION_NORWAY, super::KEY_NATION_POLAND, super::KEY_NATION_PORTUGAL,
        super::KEY_NATION_RUSSIA, super::KEY_NATION_SCOTLAND, super::KEY_NATION_SPAIN, super::KEY_NATION_SWEDEN,
        super::KEY_NATION_TURKEY, super::KEY_NATION_UNITED_STATES, super::KEY_NATION_WALES, super::KEY_NATION_SOUTH_KOREA,
    ];
}

/// `try_add_entry` through the exe (the detour runs its comparer in comparison mode).
unsafe fn hall_of_fame_try_add(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x5ef5e0usize);
    f(this, table, staff, score)
}

/// Whether the person is one of the 16 human managers (the last 16 staff rows).
unsafe fn is_human_manager_row(staff: usize) -> bool {
    read_i32(staff) >= read_i32(G_STAFF_COUNT) - 16
}

/// Comparer of the hall of fame entry routines: side by side over the 28 tables when the person is a human manager
/// (otherwise nothing is written and the return value is compared).
unsafe fn compare_hall_of_fame_entry(this: usize, staff: usize, what: &str, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    use hall_of_fame_consts as hf;
    if staff == 0 || !is_human_manager_row(staff) {
        let r = original();
        let n = copy();
        SCOREBOARD.record(r == n, || format!("{}(staff {:x}) original={} rust={}", what, staff, r, n));
        return r;
    }
    let regions = [(this, hf::TABLES * hf::TABLE_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, copy);
    SCOREBOARD.record(equal && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("{}(staff {:x}) original={} rust={}, tables differ from byte {:#x}", what, staff, r, n, k)
    });
    r
}

/// Puts a human manager's score in the calendar year table (0) of the hall of fame.
///
/// `HallOfFame::add_calendar_year_entry(staff, score)`. Without a person, the game's error box (line 0xff) and 0; a
/// person who is not one of the 16 human managers gives 0; otherwise `try_add_entry` with table 0.
/// Original: `HallOfFame::add_calendar_year_entry` (0x5ef270, hall_of_fame.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_calendar_year_entry(this: usize, staff: usize, score: u32) -> u32 {
    if staff == 0 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0xff);
        return 0;
    }
    if !is_human_manager_row(staff) {
        return 0;
    }
    covered!("mwg_7 calendar year entry");
    hall_of_fame_try_add(this, 0, staff, score) // mwg7: the calendar year table
}

/// Comparer for `add_calendar_year_entry`.
pub(crate) unsafe extern "thiscall" fn compare_add_calendar_year_entry(this: usize, staff: usize, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    compare_hall_of_fame_entry(this, staff, "add_calendar_year_entry", || o(this, staff, score), || add_calendar_year_entry(this, staff, score))
}

/// Puts a human manager's score in the hall of fame table of a league's nation.
///
/// `HallOfFame::add_league_entry(staff, nation_id, score)`. Without a person or with a nation id outside the nation
/// table (`G_NATION_COUNT`), the game's error box (line 0x11b) and 0; a person who is not one of the 16 human managers
/// gives 0. The table is the position (1..27) of the nation in `LEAGUE_NATIONS`, −1 for any other nation (which
/// `try_add_entry` refuses with its error box).
/// Original: `HallOfFame::add_league_entry` (0x5ef330, hall_of_fame.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_league_entry(this: usize, staff: usize, nation_id: i32, score: u32) -> u32 {
    if staff == 0 || nation_id < 0 || nation_id > read_i32(G_NATION_COUNT) - 1 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x11b);
        return 0;
    }
    if !is_human_manager_row(staff) {
        return 0;
    }
    covered!("mwg_7 league entry");
    let table = hall_of_fame_consts::LEAGUE_NATIONS
        .iter()
        .position(|&k| nation_id == read_i32(k))
        .map_or(0xffff_ffff, |i| i as u32 + 1); // mwg7: the league's table
    hall_of_fame_try_add(this, table, staff, score)
}

/// Comparer for `add_league_entry`.
pub(crate) unsafe extern "thiscall" fn compare_add_league_entry(this: usize, staff: usize, nation_id: i32, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_hall_of_fame_entry(this, staff, "add_league_entry", || o(this, staff, nation_id, score), || add_league_entry(this, staff, nation_id, score))
}

/// Puts a manager's score in one of the 28 hall of fame tables when it beats the lowest entry, and keeps the table
/// sorted.
///
/// `HallOfFame::try_add_entry(table, staff, score)` (the table is read as a byte). A table outside 0..27 or no person
/// shows the game's error box (line 0x13b) and gives 0. A score of 0, or not above the last (100th) entry's, gives 0.
/// The new entry: the manager's name (`exe::database_format_staff_name`, style 1, 50 characters), "club/nation" from
/// his club (`.club_job()`) and national team (`.national_job()`) names (`exe::club_get_national_team_name`), the one
/// he has, or "No Club/Nation"; the time stamp (time(0)), the score, the game's random seed (`G_GAME_RANDOM_SEED`) and
/// today. An entry among the first 99 with the same name, seed and date is a repeat: 0. Otherwise the new entry
/// replaces the last one, the table is sorted again (qsort with `hall_of_fame_compare_entries`: score high to low,
/// then older first) and 1 is returned.
/// Original: `HallOfFame::try_add_entry` (0x5ef5e0, hall_of_fame.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn try_add_entry(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    use hall_of_fame_consts as hf;
    let mut today = [0u8; 8];
    exe::date_date_54a110(today.as_mut_ptr() as usize);
    let t = table as u8 as i8;
    if !(0..=0x1b).contains(&t) || staff == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x13b);
        return 0;
    }
    let base = this + t as usize * hf::TABLE_SIZE;
    let last = base + (hf::ENTRIES - 1) * hf::ENTRY_SIZE;
    if score == 0 || score <= read_u32(last + hf::SCORE) {
        return 0;
    }
    covered!("mwg_7 entry beats the last one");
    let strncpy: unsafe extern "C" fn(*mut u8, *const u8, u32) -> usize = core::mem::transmute(hf::STRNCPY);
    let time: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(hf::TIME_FN);
    let mut text = [0u8; 200];
    let mut name = [0u8; hf::NAME_LEN];
    let mut club_text = [0u8; hf::NAME_LEN];
    exe::database_format_staff_name(staff, text.as_mut_ptr() as usize, 200, 1);
    strncpy(name.as_mut_ptr(), text.as_ptr(), hf::NAME_LEN as u32);
    name[hf::NAME_LEN - 1] = 0;
    let person = Staff(staff);
    let mut club_name = [0u8; 200];
    let mut nation_name = [0u8; 200];
    if !person.club_job().is_null() {
        exe::club_get_national_team_name(person.club_job(), 0usize, club_name.as_mut_ptr() as usize, 200, 1, 1);
    }
    if !person.national_job().is_null() {
        exe::club_get_national_team_name(person.national_job(), 0usize, nation_name.as_mut_ptr() as usize, 200, 1, 1);
    }
    let (c, n) = (club_name.as_ptr() as usize, nation_name.as_ptr() as usize);
    let out = text.as_mut_ptr() as usize;
    match (club_name[0] != 0, nation_name[0] != 0) {
        (true, true) => {
            let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            format(out, hf::CLUB_AND_NATION, c, n);
        }
        (true, false) => {
            strncpy(text.as_mut_ptr(), club_name.as_ptr(), 200);
        }
        (false, true) => {
            strncpy(text.as_mut_ptr(), nation_name.as_ptr(), 200);
        }
        (false, false) => {
            let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            translate(out, hf::NO_CLUB_OR_NATION);
        }
    }
    strncpy(club_text.as_mut_ptr(), text.as_ptr(), hf::NAME_LEN as u32);
    club_text[hf::NAME_LEN - 1] = 0;
    let stamp = time(0);
    let seed = read_u32(G_GAME_RANDOM_SEED);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(today.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    let same_name = |e: usize| {
        let mut k = 0;
        loop {
            let (a, b) = (read_u8(e + k), name[k]);
            if a != b {
                return false;
            }
            if a == 0 {
                return true;
            }
            k += 1;
        }
    };
    for i in 0..hf::ENTRIES - 1 {
        let e = base + i * hf::ENTRY_SIZE;
        if same_name(e) && read_u32(e + hf::SEED) == seed && read_u16(e + hf::DATE + 2) == u16::from_le_bytes([today[2], today[3]])
            && read_u16(e + hf::DATE) == u16::from_le_bytes([today[0], today[1]]) {
            covered!("mwg_7 repeated entry");
            return 0;
        }
    }
    core::ptr::copy_nonoverlapping(name.as_ptr(), last as *mut u8, hf::NAME_LEN);
    core::ptr::copy_nonoverlapping(club_text.as_ptr(), (last + hf::CLUB) as *mut u8, hf::NAME_LEN);
    write_u32(last + hf::TIME, stamp);
    write_u32(last + hf::SCORE, score); // mwg7: the entry's score
    write_u32(last + hf::SEED, seed);
    exe::date_operator_3d(last + hf::DATE, tmp.as_mut_ptr() as usize, today.as_ptr() as usize);
    exe::crt_qsort(base, hf::ENTRIES as u32, hf::ENTRY_SIZE as u32, hf::COMPARE_ENTRIES);
    1
}

/// Comparer for `try_add_entry`: side by side over the 28 tables.
pub(crate) unsafe extern "thiscall" fn compare_try_add_entry(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(%%I2%%);
    let regions = [(this, hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, table, staff, score), || try_add_entry(this, table, staff, score));
    SCOREBOARD.record(equal && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("try_add_entry(table {}, staff {:x}, score {}) original={} rust={}, tables differ from byte {:#x}", table as u8 as i8, staff, score, r, n, k)
    });
    r
}
