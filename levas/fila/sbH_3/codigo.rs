// ---------- sbH_3: what a date holds for a club in the friendlies (friendly.cpp) ----------

/// The answers of Friendly::get_club_date_status: the date is free, taken (or too close to something), in a
/// tournament already created (2) or not yet (3), one of the chosen dates, the day of a friendly, or the day of a
/// scheduled request.
const DATE_FREE: u8 = 1;
const DATE_TAKEN: u8 = 2;
const DATE_CHOSEN: u8 = 4;
const DATE_FRIENDLY: u8 = 5;
const DATE_REQUEST: u8 = 6;
/// The tournament kind of a request for dates (`kind` 0xb): the chosen dates are checked instead of the club's own.
const KIND_DATE_REQUEST: u8 = 0xb;
/// A node of the scheduled slot list (`friendly::SCHEDULED_SLOT_LIST`): the club +0, the slot count +8 (a signed byte)
/// and the slots from +9, 0x19 bytes each (a date +0 and the requesting club +0xc, as a friendly match record).
const SCHEDULED_SLOT_COUNT: usize = 0x8;
const SCHEDULED_SLOTS: usize = 0x9;
const SCHEDULED_SLOT_SIZE: usize = 0x19;

/// `date` moved `days` back (Date::sub_days `exe::date_sub_days`, into a new date).
unsafe fn days_before(date: usize, days: u32) -> [u8; 8] {
    let mut out = [0u8; 8];
    exe::date_sub_days(date, out.as_mut_ptr() as usize, days);
    out
}

/// `date` moved `days` on (Date::add_days `exe::date_add_days`, into a new date).
unsafe fn days_after(date: usize, days: u32) -> [u8; 8] {
    let mut out = [0u8; 8];
    exe::date_add_days(date, out.as_mut_ptr() as usize, days);
    out
}

/// The address of a local date.
fn at(d: &[u8; 8]) -> usize {
    d.as_ptr() as usize
}

/// True if `date` lies strictly between `start` and `end` (`compare_dates`: the year unsigned, then the day signed).
unsafe fn strictly_between(date: usize, start: &[u8; 8], end: &[u8; 8]) -> bool {
    compare_dates(at(start), date) < 0 && compare_dates(at(end), date) > 0
}

/// True if `date` is less than a week ahead: today (`G_CURRENT_DATE`) after `date` − 7 days.
unsafe fn sooner_than_a_week(date: usize) -> bool {
    compare_dates(G_CURRENT_DATE, at(&days_before(date, 7))) > 0
}

/// The 8-byte date `k` of the chosen dates `chosen`.
fn chosen_date(chosen: usize, k: i32) -> usize {
    chosen.wrapping_add((k as usize).wrapping_mul(8))
}

/// The tournament slots (`friendly::TOURNAMENT_SLOTS`, 127 of `tournament_slot::SIZE`) in use (`tournament_slot::HOST`)
/// with `club` among their four invited clubs (`tournament_slot::INVITED_CLUBS`).
unsafe fn club_tournament_slots(this: Friendly, club: usize) -> impl Iterator<Item = usize> {
    (0..0x7f).map(move |i| this.addr() + friendly::TOURNAMENT_SLOTS + i * 0x69).filter(move |&slot| {
        read_usize(slot + tournament_slot::HOST) != 0 && (0..4).any(|j| read_usize(slot + tournament_slot::INVITED_CLUBS + 4 * j) == club)
    })
}

/// The rest days of a tournament kind byte (`friendly_get_rest_days_for_kind`: 2 for kinds 1 and 10, 3 for kind 2, 1
/// for kinds 0, 3, 4 and 11; any other: the friendly.cpp box of line 0xf18 and 1).
unsafe fn rest_days(kind: u8) -> u32 {
    friendly_get_rest_days_for_kind(kind as u32) as u32
}

/// What `friendly_get_club_date_status` answers, worked out without side effects but for the list cursors: the value
/// and the three outputs, or the friendly match to take out of the list before asking again (`retry`).
struct ClubDateStatus {
    value: u8,
    tournament: usize,
    friendly: usize,
    request: usize,
    retry: usize,
}

impl ClubDateStatus {
    fn answer(value: u8) -> Self {
        ClubDateStatus { value, tournament: 0, friendly: 0, request: 0, retry: 0 }
    }
}

/// The tournaments: first a tournament whose window (its start date − 1 to the start + its kind's rest days) holds the
/// date strictly: the slot, and 2 if its competition exists (`tournament_slot::COMPETITION`), 3 if not. Then the wider
/// windows (the start − the rest days of `kind` − 1 to the start + its kind's rest days + 1, strictly): 2.
unsafe fn tournament_status(this: Friendly, club: usize, date: usize, kind: u32) -> Option<ClubDateStatus> {
    for slot in club_tournament_slots(this, club) {
        let rest = rest_days(read_u8(slot + tournament_slot::KIND));
        let start = days_before(slot + tournament_slot::START_DATE, 1);
        let end = days_after(slot + tournament_slot::START_DATE, rest);
        if strictly_between(date, &start, &end) {
            covered!("sbH_3 date in a tournament");
            let created = read_usize(slot + tournament_slot::COMPETITION) != 0;
            return Some(ClubDateStatus { tournament: slot, ..ClubDateStatus::answer(if created { DATE_TAKEN } else { DATE_TAKEN + 1 }) });
        }
    }
    for slot in club_tournament_slots(this, club) {
        let rest = rest_days(read_u8(slot + tournament_slot::KIND));
        let before = rest_days(kind as u8);
        let start = days_before(slot + tournament_slot::START_DATE, before + 1);
        let end = days_after(slot + tournament_slot::START_DATE, rest + 1);
        if strictly_between(date, &start, &end) {
            return Some(ClubDateStatus::answer(DATE_TAKEN));
        }
    }
    None
}

/// The friendlies (`friendly::FRIENDLY_MATCH_LIST`, its cursor rewound, `exe::llist_get_next`): the first one between
/// two clubs (`friendly_match::HOME_CLUB`, `friendly_match::AWAY_CLUB`) with `club` in it and its date within a day of
/// `date` (inclusive). On its very day, if the other club is not free then
/// (`exe::friendly_is_club_free_on_date`(date, other, 0)) the match is to be taken out and the question asked again;
/// the day before or after: 2; its day: the match and 5.
unsafe fn friendly_status(this: Friendly, club: usize, date: usize) -> Option<ClubDateStatus> {
    let list = this.friendly_match_list();
    if list.is_null() {
        return None;
    }
    list.set_cursor(list.head());
    loop {
        let m = exe::llist_get_next(this.friendly_match_list());
        if m == 0 {
            return None;
        }
        let (home, away) = (read_usize(m + friendly_match::HOME_CLUB), read_usize(m + friendly_match::AWAY_CLUB));
        if home == 0 || away == 0 {
            continue;
        }
        let other = if home == club { away } else if away == club { home } else { continue };
        let start = days_before(m + friendly_match::DATE, 1);
        let end = days_after(m + friendly_match::DATE, 1);
        if compare_dates(date, at(&start)) < 0 || compare_dates(date, at(&end)) > 0 {
            continue;
        }
        covered!("sbH_3 date near a friendly");
        if same_date(m + friendly_match::DATE, date) && exe::friendly_is_club_free_on_date(this.addr(), date as *mut [u8; 8], other, 0) == 0 {
            covered!("sbH_3 friendly with a busy opponent");
            return Some(ClubDateStatus { retry: m, ..ClubDateStatus::answer(0) });
        }
        if same_date(date, at(&start)) || same_date(date, at(&end)) {
            return Some(ClubDateStatus::answer(DATE_TAKEN));
        }
        return Some(ClubDateStatus { friendly: m, ..ClubDateStatus::answer(DATE_FRIENDLY) });
    }
}

/// The scheduled requests (`friendly::SCHEDULED_SLOT_LIST`, its cursor rewound): the first node of `club` with a slot
/// in use (its requesting club) dated within a day of `date`, the days looked at in order from the day before: on the
/// day itself the node and 6, the day before or after 2.
unsafe fn request_status(this: Friendly, club: usize, date: usize) -> Option<ClubDateStatus> {
    let list = Llist(this.scheduled_slot_list());
    if list.is_null() {
        return None;
    }
    list.set_cursor(list.head());
    loop {
        let node = exe::llist_get_next(this.scheduled_slot_list());
        if node == 0 {
            return None;
        }
        if read_usize(node) != club {
            continue;
        }
        let mut day = days_before(date, 1);
        let last = days_after(date, 1);
        while compare_dates(at(&last), at(&day)) >= 0 {
            let count = read_i8(node + SCHEDULED_SLOT_COUNT);
            let mut c: i8 = 0;
            while c < count {
                let slot = node + SCHEDULED_SLOTS + (c as isize as usize).wrapping_mul(SCHEDULED_SLOT_SIZE);
                if read_usize(slot + friendly_match::REQUESTING_CLUB) != 0 && same_date(slot, at(&day)) {
                    if same_date(date, at(&day)) {
                        return Some(ClubDateStatus { request: node, ..ClubDateStatus::answer(DATE_REQUEST) });
                    }
                    return Some(ClubDateStatus::answer(DATE_TAKEN));
                }
                c += 1;
            }
            day = days_after(at(&day), 1);
        }
    }
}

/// A request for dates (`kind` 0xb) with `n` chosen dates: the days between the date and the chosen block (from the
/// date to the first chosen − 1 when the date is before it; from the last chosen + 1 to the date + 1 when after),
/// strictly before the end, must all be free for the club (`exe::friendly_is_club_free_on_date`(day, club, 0)), or 2;
/// a chosen date the day before or after the date: 2 (on it: 4); more than three days before the first chosen date or
/// after the last: 2; less than a week ahead: 2; otherwise 1.
unsafe fn date_request_status(this: Friendly, club: usize, date: usize, chosen: usize, n: i16) -> u8 {
    let (first, last) = (chosen_date(chosen, 0), chosen_date(chosen, n as i32 - 1));
    let window = if compare_dates(first, date) > 0 {
        let mut start = [0u8; 8];
        core::ptr::copy_nonoverlapping(date as *const u8, start.as_mut_ptr(), 8);
        Some((start, days_before(first, 1)))
    } else if compare_dates(last, date) < 0 {
        Some((days_after(last, 1), days_after(date, 1)))
    } else {
        None
    };
    if let Some((mut day, end)) = window {
        while compare_dates(at(&end), at(&day)) > 0 {
            if exe::friendly_is_club_free_on_date(this.addr(), day.as_mut_ptr() as *mut [u8; 8], club, 0) == 0 {
                covered!("sbH_3 request for dates with a busy day");
                return DATE_TAKEN;
            }
            day = days_after(at(&day), 1);
        }
    }
    let (before, after) = (days_before(date, 1), days_after(date, 1));
    let mut k: i8 = 0;
    while (k as i16) < n {
        let c = chosen_date(chosen, k as i32);
        if same_date(c, at(&before)) || same_date(c, at(&after)) {
            return DATE_TAKEN;
        }
        if same_date(c, date) {
            return DATE_CHOSEN;
        }
        k = k.wrapping_add(1);
    }
    if compare_dates(at(&days_before(first, 3)), date) > 0 || compare_dates(at(&days_after(last, 3)), date) < 0 {
        return DATE_TAKEN;
    }
    if sooner_than_a_week(date) { DATE_TAKEN } else { DATE_FREE }
}

/// The whole answer of `friendly_get_club_date_status` (see there), the retry included.
#[allow(clippy::too_many_arguments)]
unsafe fn club_date_status_plan(this: Friendly, club: usize, date: usize, chosen: usize, n: i16, kind: u32, check_stadium: u32) -> ClubDateStatus {
    let mut k: i8 = 0;
    while (k as i16) < n {
        if same_date(chosen_date(chosen, k as i32), date) {
            return ClubDateStatus::answer(DATE_CHOSEN);
        }
        k = k.wrapping_add(1);
    }
    if let Some(s) = tournament_status(this, club, date, kind).or_else(|| friendly_status(this, club, date)).or_else(|| request_status(this, club, date)) {
        return s;
    }
    if kind as u8 == KIND_DATE_REQUEST {
        if n != 0 {
            return ClubDateStatus::answer(date_request_status(this, club, date, chosen, n));
        }
        if exe::friendly_is_club_free_on_date(this.addr(), date as *mut [u8; 8], club, 0) == 0 {
            return ClubDateStatus::answer(DATE_TAKEN);
        }
    } else {
        if exe::friendly_is_club_free_on_date(this.addr(), date as *mut [u8; 8], club, kind) == 0 {
            return ClubDateStatus::answer(DATE_TAKEN);
        }
        // the original pushes the day and the year words with the high halves of its registers: the year's are those
        // of `check_stadium` (1 in every caller), the day's are a leftover the callee does not read
        if check_stadium != 0 && matches!(kind as u8, 0..=2)
            && exe::fixture_manager_is_stadium_in_use(read_usize(G_FIXTURE_MANAGER), Club(club).stadium().addr(), read_u16(date) as u32,
                (check_stadium & 0xffff_0000) | read_u16(date + 2) as u32, 0xffff_ffff) != 0
        {
            covered!("sbH_3 stadium in use");
            return ClubDateStatus::answer(DATE_TAKEN);
        }
    }
    if sooner_than_a_week(date) {
        return ClubDateStatus::answer(DATE_TAKEN);
    }
    covered!("sbH_3 date free");
    ClubDateStatus::answer(DATE_FREE)
}

/// Rule: a date is open to a club for a friendly only if it is not one of the dates already chosen (4), not within
/// a tournament it plays (2, or 3 while the tournament is not set up yet) nor close to one, not within a day of a
/// friendly it plays (5 on the day itself) or of a scheduled request (6 on the day itself), the club has no match
/// there, its stadium is free (for kinds 0 to 2, when asked) and the date is at least a week ahead; a request for dates
/// (kind 11) also needs the days up to the chosen block free and the date near it. A friendly whose opponent is no
/// longer free on its day is cancelled and the question asked again.
///
/// `Friendly::get_club_date_status(club, date, &tournament, &friendly, &request, chosen, n, kind, check_stadium)`: the
/// three outputs zeroed; `club_date_status_plan`: the `n` chosen dates (8 bytes each, a signed byte index), then
/// `tournament_status`, `friendly_status`, `request_status`; a request for dates (`KIND_DATE_REQUEST`) with chosen
/// dates `date_request_status`, without them the club free on the date (kind 0); any other kind the club free on the
/// date for `kind` and, with `check_stadium` and kind 0 to 2, its stadium (`club::STADIUM`) not in use that day
/// (`exe::fixture_manager_is_stadium_in_use`([`G_FIXTURE_MANAGER`], stadium, day, year, −1)); then 2 when less than a
/// week ahead (`sooner_than_a_week`), else 1. A retry takes the friendly out of its list (`exe::llist_remove`) and
/// returns the answer of Friendly::get_club_date_status on [`G_FRIENDLY`] (not on `this`) with the same arguments and
/// `check_stadium` 1. The rest days of a bad tournament kind show the box of line 0xf18 (and count 1).
/// Original: `Friendly::get_club_date_status` (0x5c75a0, friendly.cpp, thiscall, ret 0x24).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn friendly_get_club_date_status(this: usize, club: usize, date: usize, tournament: *mut usize, friendly: *mut usize,
    request: *mut usize, chosen: usize, n: u32, kind: u32, check_stadium: u32) -> u8 {
    for out in [tournament, friendly, request] {
        *out = 0;
    }
    let s = club_date_status_plan(Friendly(this), club, date, chosen, n as i16, kind, check_stadium);
    if s.retry != 0 {
        exe::llist_remove(Friendly(this).friendly_match_list(), s.retry);
        let again: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut usize, *mut usize, usize, u32, u32, u32) -> u8 =
            core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
        return again(read_usize(G_FRIENDLY), club, date, tournament, friendly, request, chosen, n, kind, 1);
    }
    *tournament = s.tournament;
    *friendly = s.friendly;
    *request = s.request;
    s.value
}

/// Comparer for `friendly_get_club_date_status`: the copy's answer is worked out first (it only moves the list
/// cursors, which the original rewinds); when it would take a friendly out and ask again, only the original runs;
/// otherwise the value and the three outputs must match.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_friendly_get_club_date_status(this: usize, club: usize, date: usize, tournament: *mut usize, friendly: *mut usize,
    request: *mut usize, chosen: usize, n: u32, kind: u32, check_stadium: u32) -> u8 {
    let plan = club_date_status_plan(Friendly(this), club, date, chosen, n as i16, kind, check_stadium);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut usize, *mut usize, usize, u32, u32, u32) -> u8>(%%I0%%)(
        this, club, date, tournament, friendly, request, chosen, n, kind, check_stadium);
    if plan.retry != 0 {
        return r;
    }
    let got = (r, *tournament, *friendly, *request);
    SCOREBOARD.record(got == (plan.value, plan.tournament, plan.friendly, plan.request),
        || format!("friendly_get_club_date_status({:#x}, club {:#x}, date {:#x}, n {}, kind {}, check {}): original {:x?} copy {:x?}", this, club, read_u32(date), n as i16, kind as u8, check_stadium,
            got, (plan.value, plan.tournament, plan.friendly, plan.request)));
    r
}
