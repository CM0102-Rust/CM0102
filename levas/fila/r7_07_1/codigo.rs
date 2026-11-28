// --- r7_07_1: the player_setup.cfg lines applied on game creation (player_regen.cpp, 0x7a30c0 and 0x7a3a10) ---

/// TRANSFER_MANAGER's future transfer from the player_setup.cfg (0x8bf060, transfer_manager.cpp, no name in the map):
/// thiscall(person, club, date, fee, wages, contract years) ret 0x18.
const FN_TRANSFER_MANAGER_ADD_CONFIG_FUTURE_TRANSFER: usize = 0x8bf060;
/// The shared text buffer the player_setup.cfg error boxes are formatted into.
const PLAYER_SETUP_TEXT: usize = 0xde1f64;
const PLAYER_SETUP_ERROR_TITLE: usize = 0x9870e0; // "Error"
const PLAYER_SETUP_NO_CLUB: usize = 0xa6bf94; // "None"
const PLAYER_SETUP_FILE: usize = 0xa6b2e4; // "player_setup.cfg"
const PLAYER_SETUP_DATA_DIR: usize = 0x9b5348; // "CM3_DATA"
const PLAYER_SETUP_READ_MODE: usize = 0x9899fc; // "r"
const PLAYER_SETUP_PERMANENT: usize = 0xa6bd14; // "PERMANENT_"
const PLAYER_SETUP_FUTURE_PERMANENT: usize = 0xa6bd00; // "FUTURE_PERMANENT_"
const PLAYER_SETUP_UNKNOWN_TYPE: usize = 0xa6b848; // "Player config - unknown type"
/// sscanf formats.
const SCAN_DATE: usize = 0xa6bf0c; // "%hd %hd %hd"
const SCAN_SHORT: usize = 0xa6bc70; // "%hd"
const SCAN_INT: usize = 0x98806c; // "%d"
const SCAN_FUTURE_TRANSFER: usize = 0xa6b9cc; // "%hd %hd %hd %ld %ld %hd"
const SCAN_LOAN: usize = 0xa6b89c; // "%hd %hd %hd %hd %hd %hd"

/// The "found at club" / "not found" error formats of one line type ('%s' first, last, common names and the club).
#[derive(Clone, Copy)]
struct SetupNames {
    found: usize,
    missing: usize,
}
const NAMES_INJURY: SetupNames = SetupNames { found: 0xa6bf54, missing: 0xa6bf18 }; // also AWOL and the drug bans
const NAMES_MISCONDUCT: SetupNames = SetupNames { found: 0xa6be60, missing: 0xa6be20 };
const NAMES_FALSE_PASSPORT: SetupNames = SetupNames { found: 0xa6bda0, missing: 0xa6bd5c };
const NAMES_MANAGER_CONF: SetupNames = SetupNames { found: 0xa6bcb8, missing: 0xa6bc74 };
const NAMES_RETIREMENT: SetupNames = SetupNames { found: 0xa6bbc0, missing: 0xa6bb80 };
const NAMES_INT_RETIREMENT: SetupNames = SetupNames { found: 0xa6bb38, missing: 0xa6baf4 };
const NAMES_FUTURE_TRANSFER: SetupNames = SetupNames { found: 0xa6baac, missing: 0xa6ba68 };
const NAMES_LOAN: SetupNames = SetupNames { found: 0xa6b914, missing: 0xa6b8dc };
/// The "invalid parameters\n%s" formats (the rest of the line after the type).
const BAD_RETIREMENT: usize = 0xa6bed4; // also the drug and false passport bans' count error
const BAD_BAN: usize = 0xa6bea4;
const BAD_MISCONDUCT: usize = 0xa6bde8;
const BAD_FALSE_PASSPORT: usize = 0xa6bd20;
const BAD_MANAGER_CONF: usize = 0xa6bc38;
const BAD_FUTURE_TRANSFER_COUNT: usize = 0xa6b990;
const BAD_FUTURE_TRANSFER_YEAR: usize = 0xa6b954;
const BAD_LOAN: usize = 0xa6b868; // also the retirement's date error
/// The "'%s' not found" formats of the club name.
const CLUB_MISSING_MANAGER_CONF: usize = 0xa6bc04;
const CLUB_MISSING_FUTURE_TRANSFER: usize = 0xa6b9e4;
const CLUB_MISSING_LOAN: usize = 0xa6b8b4;
const NOT_A_PLAYER_FUTURE_TRANSFER: usize = 0xa6ba1c;

/// The quoted fields of one player_setup.cfg line, read by `player_setup_read_quoted_token`.
struct SetupTokens {
    kind: [u8; 0x100],
    first: [u8; 0x100],
    common: [u8; 0x100],
    last: [u8; 0x100],
    club: [u8; 0x100],
    extra: [u8; 0x100],
    /// The line after the type token (what the "invalid parameters" boxes print).
    rest: usize,
}

impl SetupTokens {
    fn new() -> Self {
        SetupTokens { kind: [0; 0x100], first: [0; 0x100], common: [0; 0x100], last: [0; 0x100], club: [0; 0x100], extra: [0; 0x100], rest: 0 }
    }
    fn first(&self) -> usize {
        self.first.as_ptr() as usize
    }
    fn common(&self) -> usize {
        self.common.as_ptr() as usize
    }
    fn last(&self) -> usize {
        self.last.as_ptr() as usize
    }
    fn club(&self) -> usize {
        self.club.as_ptr() as usize
    }
    /// The person of the line by the three names in the named club (`player_setup_find_staff`).
    unsafe fn person(&self, in_squad: u32) -> Staff {
        Staff(player_setup_find_staff(self.first(), self.common(), self.last(), self.club(), in_squad))
    }
}

/// A date of the file (day, month 0..11, year).
#[derive(Clone, Copy, PartialEq, Debug)]
struct SetupDate {
    day: i16,
    month: i16,
    year: i16,
}

impl SetupDate {
    fn valid_day_month(self) -> bool {
        (1..=31).contains(&self.day) && (0..=11).contains(&self.month)
    }
    /// `Date::Date(day, month, year, -1)`.
    unsafe fn date(self) -> [u16; 4] {
        let mut d = [0u16; 4];
        exe::date_date_549f70(&mut d, self.day as i32 as u32, self.month as i32 as u32, self.year as i32 as u32, u32::MAX);
        d
    }
}

/// What one line asks for, decided before anything is changed.
enum SetupAction {
    Nothing,
    /// An error box: the format and up to four '%s' arguments.
    Report(usize, [usize; 4]),
    Absence(Staff),
    SetFlag(usize),
    MarkAllRegen,
    Ban { person: Staff, reason: u8, until: SetupDate },
    Injury { person: Staff, kind: u8, days: i32, future: u32 },
    BoardRatings { club: Club, value: i16 },
    Retirement { person: Staff, when: SetupDate },
    IntRetirement(Staff),
    FutureTransfer { person: Staff, club: Club, when: SetupDate, fee: i32, wages: i32, years: i16 },
    Loan { person: Staff, club: Club, start: SetupDate, end: SetupDate },
}

/// Two-digit years: below 10 is 20xx, below 100 is 19xx.
fn setup_year(y: i16) -> i16 {
    if y < 10 {
        y.wrapping_add(2000)
    } else if y < 100 {
        y.wrapping_add(1900)
    } else {
        y
    }
}

/// The club whose English name matches (51 characters), or 0.
unsafe fn club_by_english_name(name: usize) -> usize {
    for k in 0..read_i32(G_CLUB_COUNT).max(0) {
        let c = club_at(k as isize);
        if strncmp(exe::club_get_english_name(c.addr()), name, 0x33) == 0 {
            return c.addr();
        }
    }
    0
}

/// An "invalid parameters" box with the rest of the line, only when reporting.
fn setup_bad(report: u32, format: usize, rest: usize) -> SetupAction {
    if report == 0 {
        return SetupAction::Nothing;
    }
    SetupAction::Report(format, [rest, 0, 0, 0])
}

/// The person of the line was not found: when reporting, looks for the names in every club and boxes where they are
/// ("found at club", the club or "None") or that they are nowhere.
unsafe fn setup_missing_person(t: &SetupTokens, report: u32, in_squad: u32, names: SetupNames) -> SetupAction {
    if report == 0 {
        return SetupAction::Nothing;
    }
    let p = Staff(player_setup_find_staff(t.first(), t.common(), t.last(), 0, in_squad));
    if p.is_null() {
        return SetupAction::Report(names.missing, [t.first(), t.last(), t.common(), t.club()]);
    }
    let club = p.club_job();
    let where_ = if club.is_null() { PLAYER_SETUP_NO_CLUB } else { club.addr() + club::NAME };
    SetupAction::Report(names.found, [t.first(), t.last(), t.common(), where_])
}

/// DRUGS/NANDROLONE (reason 5), MISCONDUCT (3) and FALSE_PASSPORT (6): a ban by days until the date (day month year)
/// after the names. The year (two digits: 20xx or 19xx, applied twice) up to 2010 and not before the current year.
/// Applied in report mode too.
unsafe fn decode_setup_ban(t: &SetupTokens, tail: usize, report: u32, reason: u8, names: SetupNames, bad_count: usize, bad_date: usize) -> SetupAction {
    let person = t.person(1);
    if person.is_null() {
        return setup_missing_person(t, report, 1, names);
    }
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let (mut day, mut month, mut year) = (0i16, 0i16, 0i16);
    if scan(tail, SCAN_DATE, &mut day as *mut i16, &mut month as *mut i16, &mut year as *mut i16) != 3 {
        return setup_bad(report, bad_count, t.rest);
    }
    let until = SetupDate { day, month: month.wrapping_sub(1), year: setup_year(setup_year(year)) };
    if !until.valid_day_month() || until.year > 2010 || (until.year as i32) < read_u16(G_CURRENT_DATE + 2) as i32 {
        return setup_bad(report, bad_date, t.rest);
    }
    if read_usize(G_DISCIPLINE) == 0 {
        return SetupAction::Nothing;
    }
    covered!("r7_07_1 ban");
    SetupAction::Ban { person, reason, until }
}

/// INJURY: the injury by name (`player_setup_injury_from_name`); "PERMANENT_<name> n" (and "FUTURE_PERMANENT_<name>
/// n", only as a recurrence) with the number after it. Only on players, never in report mode.
unsafe fn decode_setup_injury(t: &mut SetupTokens, tail: usize, report: u32) -> SetupAction {
    let person = t.person(1);
    if person.is_null() {
        return setup_missing_person(t, report, 1, NAMES_INJURY);
    }
    let name = t.extra.as_mut_ptr() as usize;
    let after = player_setup_read_quoted_token(tail, name);
    if report != 0 || person.player().is_null() {
        return SetupAction::Nothing;
    }
    covered!("r7_07_1 injury");
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let mut days = 0i32; // the original leaves it unset when sscanf meets the end of the line (-1)
    if exe::crt_strstr(name, PLAYER_SETUP_PERMANENT) != 0 && scan(after, SCAN_INT, &mut days as *mut i32) != 0 {
        let (skip, future) = if exe::crt_strstr(name, PLAYER_SETUP_FUTURE_PERMANENT) != 0 { (0x11, 1) } else { (0xa, 0) };
        let kind = player_setup_injury_from_name(name + skip) as u8;
        return SetupAction::Injury { person, kind, days, future };
    }
    SetupAction::Injury { person, kind: player_setup_injury_from_name(name) as u8, days: 0, future: 0 }
}

/// MANAGER_CONF: the board's four ratings of the club ("%hd") when the person found (squad not searched) is its manager.
unsafe fn decode_setup_manager_conf(t: &SetupTokens, tail: usize, report: u32) -> SetupAction {
    let person = t.person(0);
    if person.is_null() {
        return setup_missing_person(t, report, 0, NAMES_MANAGER_CONF);
    }
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let mut value = 0i16;
    if scan(tail, SCAN_SHORT, &mut value as *mut i16) == 0 {
        return setup_bad(report, BAD_MANAGER_CONF, t.rest);
    }
    let club = Club(club_by_english_name(t.club()));
    if club.is_null() {
        return if report == 0 { SetupAction::Nothing } else { SetupAction::Report(CLUB_MISSING_MANAGER_CONF, [t.club(), 0, 0, 0]) };
    }
    if report != 0 || read_usize(club.addr() + club::MANAGER) != person.addr() {
        return SetupAction::Nothing;
    }
    SetupAction::BoardRatings { club, value }
}

/// RETIREMENT: the retirement announced today for the date (1980 to 2010) after the names.
unsafe fn decode_setup_retirement(t: &SetupTokens, tail: usize, report: u32) -> SetupAction {
    let person = t.person(1);
    if person.is_null() {
        return setup_missing_person(t, report, 1, NAMES_RETIREMENT);
    }
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let (mut day, mut month, mut year) = (0i16, 0i16, 0i16);
    if scan(tail, SCAN_DATE, &mut day as *mut i16, &mut month as *mut i16, &mut year as *mut i16) != 3 {
        return setup_bad(report, BAD_RETIREMENT, t.rest);
    }
    let when = SetupDate { day, month: month.wrapping_sub(1), year: setup_year(setup_year(year)) };
    if !when.valid_day_month() || !(1980..=2010).contains(&when.year) {
        return setup_bad(report, BAD_LOAN, t.rest);
    }
    if report != 0 || read_usize(G_PLAYER_REGEN) == 0 {
        return SetupAction::Nothing;
    }
    SetupAction::Retirement { person, when }
}

/// FUTURE_TRANSFER: "club" day month year fee wages years (contract up to 10 years, year 1980 to 2010), on players.
unsafe fn decode_setup_future_transfer(t: &mut SetupTokens, tail: usize, report: u32) -> SetupAction {
    let person = t.person(1);
    if person.is_null() {
        return setup_missing_person(t, report, 1, NAMES_FUTURE_TRANSFER);
    }
    if person.player().is_null() {
        return if report == 0 { SetupAction::Nothing } else { SetupAction::Report(NOT_A_PLAYER_FUTURE_TRANSFER, [t.first(), t.last(), t.common(), t.club()]) };
    }
    let after = player_setup_read_quoted_token(tail, t.club()); // the buying club replaces the club name
    let club = Club(club_by_english_name(t.club()));
    if club.is_null() {
        return if report == 0 { SetupAction::Nothing } else { SetupAction::Report(CLUB_MISSING_FUTURE_TRANSFER, [t.club(), 0, 0, 0]) };
    }
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let (mut day, mut month, mut year, mut fee, mut wages, mut years) = (0i16, 0i16, 0i16, 0i32, 0i32, 0i16);
    let n = scan(after, SCAN_FUTURE_TRANSFER, &mut day as *mut i16, &mut month as *mut i16, &mut year as *mut i16, &mut fee as *mut i32,
        &mut wages as *mut i32, &mut years as *mut i16);
    let when = SetupDate { day, month: month.wrapping_sub(1), year: setup_year(year) };
    if n != 6 || !when.valid_day_month() || years > 10 {
        return setup_bad(report, BAD_FUTURE_TRANSFER_COUNT, t.rest);
    }
    if !(1980..=2010).contains(&when.year) {
        return setup_bad(report, BAD_FUTURE_TRANSFER_YEAR, t.rest);
    }
    if report != 0 {
        return SetupAction::Nothing;
    }
    SetupAction::FutureTransfer { person, club, when, fee, wages, years }
}

/// LOAN: "club" and the start and end dates (years 1980 to 2010).
unsafe fn decode_setup_loan(t: &mut SetupTokens, tail: usize, report: u32) -> SetupAction {
    let person = t.person(1);
    if person.is_null() {
        return setup_missing_person(t, report, 1, NAMES_LOAN);
    }
    let after = player_setup_read_quoted_token(tail, t.club()); // the loan club replaces the club name
    let club = Club(club_by_english_name(t.club()));
    if club.is_null() {
        return if report == 0 { SetupAction::Nothing } else { SetupAction::Report(CLUB_MISSING_LOAN, [t.club(), 0, 0, 0]) };
    }
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let mut v = [0i16; 6]; // day, month, year of the start, then of the end
    let p = v.as_mut_ptr();
    if scan(after, SCAN_LOAN, p, p.add(1), p.add(2), p.add(3), p.add(4), p.add(5)) != 6 {
        return setup_bad(report, BAD_LOAN, t.rest);
    }
    let start = SetupDate { day: v[0], month: v[1].wrapping_sub(1), year: setup_year(v[2]) };
    let end = SetupDate { day: v[3], month: v[4].wrapping_sub(1), year: setup_year(v[5]) };
    if !start.valid_day_month() || !(1980..=2010).contains(&start.year) || !end.valid_day_month() || !(1980..=2010).contains(&end.year) {
        return setup_bad(report, BAD_LOAN, t.rest);
    }
    if report != 0 {
        return SetupAction::Nothing;
    }
    covered!("r7_07_1 loan");
    SetupAction::Loan { person, club, start, end }
}

/// Reads the fields of one line and decides what it asks for: the type (`player_setup_type_from_name`), skipped when
/// outside the mask unless reporting, then the names and the type's own fields.
unsafe fn decode_player_setup_line(line: usize, mask: u32, report: u32, t: &mut SetupTokens) -> SetupAction {
    t.rest = player_setup_read_quoted_token(line, t.kind.as_mut_ptr() as usize);
    let kind = player_setup_type_from_name(t.kind.as_ptr() as usize);
    if kind & mask == 0 && report == 0 {
        return SetupAction::Nothing;
    }
    let p = player_setup_read_quoted_token(t.rest, t.first());
    let p = player_setup_read_quoted_token(p, t.common());
    let p = player_setup_read_quoted_token(p, t.last());
    let tail = player_setup_read_quoted_token(p, t.club());
    match kind {
        0x40 => {
            // AWOL, applied in report mode too; its error boxes are the INJURY ones
            let person = t.person(1);
            if person.is_null() {
                return setup_missing_person(t, report, 1, NAMES_INJURY);
            }
            if read_usize(G_AWOL) == 0 { SetupAction::Nothing } else { SetupAction::Absence(person) }
        }
        0x2000 => SetupAction::SetFlag(TACTICS_PRESETS_ON),
        0x20 => {
            if read_usize(G_STAFF_STATE_TABLE) == 0 { SetupAction::Nothing } else { SetupAction::MarkAllRegen }
        }
        0x400 => SetupAction::SetFlag(G_PLAYER_SETUP_COLOURBLIND_FLAG),
        0x100 => decode_setup_ban(t, tail, report, 5, NAMES_INJURY, BAD_RETIREMENT, BAD_BAN),
        0x1000 => decode_setup_ban(t, tail, report, 3, NAMES_MISCONDUCT, BAD_MISCONDUCT, BAD_MISCONDUCT),
        0x800 => decode_setup_ban(t, tail, report, 6, NAMES_FALSE_PASSPORT, BAD_RETIREMENT, BAD_FALSE_PASSPORT),
        0x80 => SetupAction::SetFlag(G_PLAYER_SETUP_GERMAN_FLAG),
        1 => decode_setup_injury(t, tail, report),
        2 => decode_setup_manager_conf(t, tail, report),
        0x200 => decode_setup_retirement(t, tail, report),
        4 => {
            let person = t.person(1);
            if person.is_null() {
                return setup_missing_person(t, report, 1, NAMES_INT_RETIREMENT);
            }
            if report != 0 { SetupAction::Nothing } else { SetupAction::IntRetirement(person) }
        }
        8 => decode_setup_future_transfer(t, tail, report),
        0x10 => decode_setup_loan(t, tail, report),
        _ => SetupAction::Report(PLAYER_SETUP_UNKNOWN_TYPE, [0; 4]),
    }
}

/// The ban record of a player_setup.cfg ban: by days (matches 0xff) from today until the date, the reason, only the
/// fourth rule (0, FIFA), flags | 6 (the other flag bits are the stack's in the original; `BanRecord::operator=` copies
/// only bits 0..2 and `add_ban` sets bit 0).
unsafe fn setup_ban_record(reason: u8, until: SetupDate) -> [u8; 0x14] {
    let mut ban = [0u8; 0x14];
    let b = ban.as_mut_ptr() as usize;
    exe::date_date_54a110(b + ban_record::START_DATE);
    let mut end = until.date();
    write_u8(b + ban_record::MATCHES, 0xff);
    write_u16(b + ban_record::DAYS, exe::date_days_since(end.as_ptr() as usize, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) as u16);
    write_u8(b + ban_record::MATCHES_SERVED, 0);
    exe::date_operator_3d(b + ban_record::START_DATE, end.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(b + ban_record::REASON, reason);
    write_u16(b + ban_record::REASON_AMOUNT, 0);
    for (k, rule) in [0xffu8, 0xff, 0xff, 0].into_iter().enumerate() {
        write_u8(b + ban_record::RULES + k, rule); // r7_07_1: the FIFA rule only
    }
    write_u8(b + ban_record::FLAGS, read_u8(b + ban_record::FLAGS) | 6);
    ban
}

/// The pending retirement of a RETIREMENT line: announced today (`retirement_entry::ANNOUNCED` 1), retiring on the
/// date, reason 0x20, with the person's club id (-1 without a club).
unsafe fn setup_retirement_record(person: Staff, when: SetupDate) -> [u8; retirement_entry::SIZE] {
    let mut r = [0u8; retirement_entry::SIZE];
    let a = r.as_mut_ptr() as usize;
    exe::date_date_54a110(a + retirement_entry::ANNOUNCE_DATE);
    exe::date_date_54a110(a + retirement_entry::RETIRE_DATE);
    let mut scratch = [0u16; 4];
    exe::date_operator_3d(a + retirement_entry::ANNOUNCE_DATE, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(a + retirement_entry::ANNOUNCED, 1);
    let date = when.date();
    let club = person.club_job();
    write_u32(a + retirement_entry::CLUB_ID, if club.is_null() { u32::MAX } else { read_u32(club.addr()) });
    write_u32(a, read_u32(person.addr()));
    exe::date_operator_3d(a + retirement_entry::RETIRE_DATE, scratch.as_mut_ptr() as usize, date.as_ptr() as usize);
    write_u8(a + retirement_entry::REASON, 0x20);
    r
}

/// Carries out what a line asked for.
unsafe fn apply_player_setup_action(action: &SetupAction) {
    match *action {
        SetupAction::Nothing => {}
        SetupAction::Report(format, a) => {
            let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            crt_format(PLAYER_SETUP_TEXT, format, a[0], a[1], a[2], a[3]);
            exe::gui_show_message_box(PLAYER_SETUP_ERROR_TITLE, PLAYER_SETUP_TEXT, 0);
        }
        SetupAction::Absence(person) => awol_start_absence(read_usize(G_AWOL), person.addr(), 0, 0),
        SetupAction::SetFlag(flag) => write_u8(flag, 1),
        SetupAction::MarkAllRegen => {
            covered!("r7_07_1 mark every person");
            let mut i = 0;
            while i < read_i32(G_STAFF_COUNT) {
                let f = read_usize(G_STAFF_STATE_TABLE) + i as usize * staff_state::SIZE + staff_state::STATUS_FLAGS;
                write_u8(f, read_u8(f) | staff_state_status_flags::REGEN_MARK);
                i += 1;
            }
        }
        SetupAction::Ban { person, reason, until } => {
            let mut ban = setup_ban_record(reason, until);
            add_ban(read_usize(G_DISCIPLINE), read_u32(person.addr()), ban.as_mut_ptr() as usize);
        }
        SetupAction::Injury { person, kind, days, future } => {
            add_injury(read_usize(G_INJURY), kind as u32, 0, person.addr(), 0, 1, u32::MAX, days as u32, future);
        }
        SetupAction::BoardRatings { club, value } => set_board_ratings(read_usize(G_MANAGER_MANAGER), club.addr(), value as i32 as u32),
        SetupAction::Retirement { person, when } => {
            let mut r = setup_retirement_record(person, when);
            add_retirement(read_usize(G_PLAYER_REGEN), r.as_mut_ptr() as usize);
            let f = retirement_flags_byte(read_u32(person.addr()) as i32);
            *f = *f & !staff_state_status_flags::RETIRED | staff_state_status_flags::RETIREMENT_ANNOUNCED;
        }
        SetupAction::IntRetirement(person) => {
            staff_add_disliked_club(person.addr(), main_national_team(person.nation().addr()), 1);
        }
        SetupAction::FutureTransfer { person, club, when, fee, wages, years } => {
            let mut date = when.date();
            let add: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_CONFIG_FUTURE_TRANSFER);
            add(read_usize(G_TRANSFER_MANAGER), person.addr(), club.addr(), date.as_mut_ptr() as usize, fee, wages, years as i32 as u32);
        }
        SetupAction::Loan { person, club, start, end } => {
            let (mut s, mut e) = (start.date(), end.date());
            add_config_loan(read_usize(G_TRANSFER_MANAGER), person.addr(), club.addr(), s.as_mut_ptr() as usize, e.as_mut_ptr() as usize);
        }
    }
}

/// Reads the next line of the player_setup.cfg (`exe::utils_read_config_line`, comments skipped) into `line` and
/// applies it; returns 0 at the end of the file, otherwise 1.
///
/// `player_setup_apply_line(file, line, mask, report)`: the line's type (`player_setup_type_from_name`) is applied when
/// it is in the mask, or always when reporting. The names (first, common, last, club; `player_setup_find_staff`, in the
/// club's squad and manager except MANAGER_CONF, which skips the squad) pick the person; not found, only when
/// reporting, an "Error" box saying in which club the names are or that they are nowhere. By type:
/// - AWOL (0x40): the person goes absent (`awol_start_absence`, with the AWOL object), in report mode too;
/// - PRESET_TACTICS (0x2000), COLOURBLIND (0x400), GERMAN (0x80): their flag is raised (0xb7434a, 0xb74349, 0xb74348);
/// - ARSE (0x20): every person gets the regen mark (`staff_state_status_flags::REGEN_MARK`);
/// - DRUGS and NANDROLONE (0x100), MISCONDUCT (0x1000), FALSE_PASSPORT (0x800): a ban by days until the date (reason
///   5, 3, 6; `add_ban` with the FIFA rule), year up to 2010 and not before the current one, in report mode too;
/// - INJURY (1), only for players outside report mode: the injury by name (`player_setup_injury_from_name`), or
///   "PERMANENT_<name> n" with n, or "FUTURE_PERMANENT_<name> n" as a recurrence only (`add_injury`);
/// - MANAGER_CONF (2): the four board ratings ("%hd", `set_board_ratings`) of the club when the person is its manager;
/// - RETIREMENT (0x200): a pending retirement on the date (`add_retirement`, reason 0x20, announced today) and the
///   person's retirement state "announced";
/// - INT_RETIREMENT (4): the person's national side among his disliked clubs (`staff_add_disliked_club`, forced);
/// - FUTURE_TRANSFER (8), players only: the transfer to the club on the date for the fee, wages and years (0x8bf060);
/// - LOAN (0x10): the loan to the club between the two dates (`add_config_loan`);
/// - any other type: the "Player config - unknown type" box.
/// Dates are "day month year" with a two-digit year read as 20xx below 10 and 19xx below 100 (twice for the bans and
/// the retirement), months from 1.
/// Defects of the original: AWOL and the drug bans box with the CFG_INJURY texts, the drug and false passport bans'
/// count error with the CFG_RETIREMENT one and the retirement's date error with the CFG_LOAN one; a sscanf that meets
/// the end of the line (-1) passes as read, leaving the stack's values (MANAGER_CONF, the PERMANENT_ number); the
/// ban's flag bits 3..7 and the injury type's upper bytes are the stack's.
/// Original: `player_setup_apply_line` (0x7a3a10, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_apply_line(file: usize, line: usize, mask: u32, report: u32) -> u8 {
    if exe::utils_read_config_line(file, line as *mut u8) == 0 {
        return 0;
    }
    let mut t = SetupTokens::new();
    let action = decode_player_setup_line(line, mask, report, &mut t);
    apply_player_setup_action(&action);
    1
}

/// Applies every line of CM3_DATA\player_setup.cfg whose type is in the mask (`player_setup_apply_line`), until the
/// end of the file; without the file, nothing.
/// Original: `player_setup_cfg_load` (0x7a30c0, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_cfg_load(mask: u32, report: u32) {
    let file = exe::crt_fopen(exe::utils_build_file_path(PLAYER_SETUP_DATA_DIR, PLAYER_SETUP_FILE), PLAYER_SETUP_READ_MODE);
    if file == 0 {
        return;
    }
    covered!("r7_07_1 player setup file");
    let mut line = [0u8; 0x3e8];
    while player_setup_apply_line(file, line.as_mut_ptr() as usize, mask, report) != 0 {}
    exe::crt_fclose(file);
}

/// What the comparer reads before the original runs, to check the effect after it.
enum SetupBefore {
    Nothing,
    /// The person already had a ban under the FIFA rule (`add_ban` extends it).
    Ban { banned: bool, ban: [u8; 0x14] },
    /// The person's current injury before (`add_injury` refuses a second one).
    Injury { current: i32 },
    Retirement { count: usize },
    Loan { allowed: bool, before: usize },
}

unsafe fn player_setup_before(action: &SetupAction) -> SetupBefore {
    match *action {
        SetupAction::Ban { person, reason, until } => {
            let d = read_usize(G_DISCIPLINE);
            let e = ruling_body_entry(ruling_body_of_rule(d, 0), read_u32(person.addr()));
            SetupBefore::Ban { banned: e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) > -1, ban: setup_ban_record(reason, until) }
        }
        SetupAction::Injury { person, .. } => {
            SetupBefore::Injury { current: PhysicalRecord(physical_record(read_usize(G_INJURY), read_usize(person.addr()))).current_injury() }
        }
        SetupAction::Retirement { .. } => SetupBefore::Retirement { count: read_usize(read_usize(G_PLAYER_REGEN) + player_regen_fields::RETIREMENT_COUNT) },
        SetupAction::Loan { person, club, start, end } => {
            let (s, e) = (start.date(), end.date());
            SetupBefore::Loan {
                allowed: config_loan_allowed(person.addr(), club.addr(), s.as_ptr() as usize, e.as_ptr() as usize).is_some(),
                before: exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), person.addr()),
            }
        }
        _ => SetupBefore::Nothing,
    }
}

/// Whether the original did what the copy decided, by the state it leaves (the callees have their own comparers).
unsafe fn player_setup_effect_ok(action: &SetupAction, before: &SetupBefore) -> bool {
    match (action, before) {
        (SetupAction::SetFlag(flag), _) => read_u8(*flag) == 1,
        (SetupAction::MarkAllRegen, _) => {
            let n = read_i32(G_STAFF_COUNT);
            n <= 0 || (0..n).all(|i| read_u8(read_usize(G_STAFF_STATE_TABLE) + i as usize * staff_state::SIZE + staff_state::STATUS_FLAGS) & staff_state_status_flags::REGEN_MARK != 0)
        }
        (SetupAction::Ban { .. }, SetupBefore::Ban { banned, ban }) => {
            let d = read_usize(G_DISCIPLINE);
            let b = ban.as_ptr() as usize;
            *banned || (0..read_i16(d + discipline_fields::BAN_COUNT).max(0) as usize).any(|i| {
                let s = read_usize(d + discipline_fields::BANS) + i * 20;
                read_u8(s) == 0xff && read_i16(s + ban_record::DAYS) == read_i16(b + ban_record::DAYS) && read_u8(s + ban_record::REASON) == read_u8(b + ban_record::REASON)
                    && read_u32(s + ban_record::RULES) == read_u32(b + ban_record::RULES)
            })
        }
        (SetupAction::Injury { person, kind, future, .. }, SetupBefore::Injury { current }) => {
            if *future != 0 || *current != -1 {
                return true;
            }
            let reg = PhysicalRecord(physical_record(read_usize(G_INJURY), read_usize(person.addr())));
            let last = reg.last_injury_type();
            (reg.current_injury() != -1 || read_usize(read_usize(G_INJURY) + injury_fields::INJURY_ENTRIES) == 0) && (last == *kind || (0x4d..=0x4f).contains(&last))
        }
        (SetupAction::BoardRatings { club, value }, _) => {
            read_u16(read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(club.addr()) as u32).wrapping_mul(0x49) as usize) + 6) == *value as u16
        }
        (SetupAction::Retirement { person, .. }, SetupBefore::Retirement { count }) => {
            let regen = read_usize(G_PLAYER_REGEN);
            let n = read_usize(regen + player_regen_fields::RETIREMENT_COUNT);
            let item = read_usize(regen + player_regen_fields::RETIREMENTS) + n.wrapping_sub(1) * retirement_entry::SIZE;
            n == count + 1 && read_u32(item) == read_u32(person.addr())
                && *retirement_flags_byte(read_u32(person.addr()) as i32) & staff_state_status_flags::RETIREMENT_STATE == staff_state_status_flags::RETIREMENT_ANNOUNCED
        }
        (SetupAction::Loan { person, .. }, SetupBefore::Loan { allowed, before }) => {
            let has = exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), person.addr());
            if *allowed { has != 0 } else { has == *before }
        }
        _ => true,
    }
}

/// A short name of the action for the comparer's message.
fn player_setup_action_name(action: &SetupAction) -> &'static str {
    match action {
        SetupAction::Nothing => "nothing",
        SetupAction::Report(..) => "report",
        SetupAction::Absence(_) => "absence",
        SetupAction::SetFlag(_) => "flag",
        SetupAction::MarkAllRegen => "mark all",
        SetupAction::Ban { .. } => "ban",
        SetupAction::Injury { .. } => "injury",
        SetupAction::BoardRatings { .. } => "board ratings",
        SetupAction::Retirement { .. } => "retirement",
        SetupAction::IntRetirement(_) => "international retirement",
        SetupAction::FutureTransfer { .. } => "future transfer",
        SetupAction::Loan { .. } => "loan",
    }
}

/// Comparer for `player_setup_apply_line`: the line can only be applied once, so the comparer reads the same line first
/// (ftell, `exe::utils_read_config_line`, fseek back), decides with the copy's decoder, runs the original and checks
/// that it read the same line and left the decided effect (flags, regen marks, the ban, the injury, the board ratings,
/// the retirement, the loan). In report mode (the error boxes) only the original.
pub(crate) unsafe extern "C" fn compare_player_setup_apply_line(file: usize, line: usize, mask: u32, report: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u8>(%%I0%%);
    if report != 0 || file == 0 {
        return o(file, line, mask, report);
    }
    let pos = exe::crt_ftell(file);
    let mut ahead = [0u8; 0x400];
    let has = exe::utils_read_config_line(file, ahead.as_mut_ptr()) != 0;
    exe::crt_fseek(file, pos, 0);
    let mut t = SetupTokens::new();
    let action = if has { decode_player_setup_line(ahead.as_ptr() as usize, mask, 0, &mut t) } else { SetupAction::Nothing };
    let before = player_setup_before(&action);
    let r = o(file, line, mask, report);
    let same_line = !has || strncmp(line, ahead.as_ptr() as usize, 0x400) == 0;
    let ok = r == has as u8 && same_line && player_setup_effect_ok(&action, &before);
    SCOREBOARD.record(ok, || {
        format!("player_setup_apply_line(mask {:#x}) original={} expected={} same line={} action={}", mask, r, has as u8, same_line, player_setup_action_name(&action))
    });
    r
}

/// Comparer for `player_setup_cfg_load`: opens and reads the file, so only the original runs (each line goes through
/// `compare_player_setup_apply_line`).
pub(crate) unsafe extern "C" fn compare_player_setup_cfg_load(mask: u32, report: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I1%%)(mask, report)
}
