// ---------- batch %%LEVA%%: the national teams leaving a competition and the club fixtures that clash with call-ups (national_teams.cpp) ----------

/// Every national team of a competition leaves it: for each club of the competition's list (`for_each_comp_club`,
/// stage 0) that is a national team (`exe::database_is_national_team`), its nation's current competition
/// (`national_squad_record::CURRENT_COMPETITION`) becomes −1 and, with a manager manager (`G_MANAGER_MANAGER`), the FA
/// reviews its manager's performance in the competition (`review_national_team_performance`).
/// Original: `NationalTeams::clear_competition_for_comp_clubs` (0x775420, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_competition_for_comp_clubs(this: usize, comp: usize) {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    for_each_comp_club(object, 0, |team| {
        let nation = Club(team).nation();
        if exe::database_is_national_team(team) == 0 {
            return;
        }
        covered!("sx15_6 national team leaves the competition");
        write_i32(nation_record_by_id(this, nation.addr()) + national_squad_record::CURRENT_COMPETITION, -1); // sx15_6: no current competition
        let manager_manager = read_usize(G_MANAGER_MANAGER);
        if manager_manager != 0 {
            let review: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_REVIEW_NATIONAL_TEAM_PERFORMANCE);
            review(manager_manager, comp, team);
        }
    });
}

/// `MANAGER_MANAGER::review_national_team_performance` (thiscall(manager manager, competition, national team), ret 8).
const FN_MANAGER_MANAGER_REVIEW_NATIONAL_TEAM_PERFORMANCE: usize = 0x6a5610;

/// Comparer for `clear_competition_for_comp_clubs`: the FA review draws and may sack managers (it cannot run twice),
/// so only the original runs; the plan checked afterwards: no national team of the competition has a current
/// competition.
pub(crate) unsafe extern "thiscall" fn compare_clear_competition_for_comp_clubs(this: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, comp);
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    let mut left = Vec::new();
    crate::rng::pause_trace(|| for_each_comp_club(object, 0, |team| {
        let nation = Club(team).nation();
        if exe::database_is_national_team(team) != 0 && read_i32(nation_record_by_id(this, nation.addr()) + national_squad_record::CURRENT_COMPETITION) != -1 {
            left.push(read_u32(team));
        }
    }));
    SCOREBOARD.record(left.is_empty(), || format!("clear_competition_for_comp_clubs({}): national teams still in a competition {:?}", read_i32(comp), left));
}

/// The clubs whose fixture near `date` should move: those marked with a fixture around the date
/// (`national_teams_fields::CLUB_MARKS`, 1 or 2) that have more than two players away on duty dated from the day before
/// to the day after (the call-up entries of `national_teams_fields::CLUB_CALL_UP_RECORDS`, sorted by date: the counts
/// of the entries from `before` on are added until one after `after`). Club ids below the non-national club count.
pub(crate) unsafe fn clubs_with_clashing_call_ups(this: usize, before: &[u8; 8], after: &[u8; 8]) -> Vec<usize> {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let date = |d: &[u8; 8]| (u16::from_le_bytes([d[2], d[3]]), u16::from_le_bytes([d[0], d[1]]) as i16);
    let ((by, bd), (ay, ad)) = (date(before), date(after));
    let mut clubs = Vec::new();
    for i in 0..n.max(0) as usize {
        let mark = read_u8(read_usize(this + national_teams_fields::CLUB_MARKS) + i);
        if mark != 1 && mark != 2 {
            continue;
        }
        let rec = read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS) + 6 * i;
        let entries = read_i8(rec + 5);
        let mut away: u8 = 0;
        let mut k: i8 = 0;
        while k < entries {
            let e = read_usize(rec).wrapping_add(9 * k as i32 as usize);
            let (ey, ed) = (read_u16(e + 2), read_i16(e));
            k += 1;
            if by > ey || (by == ey && bd > ed) {
                continue; // before the window
            }
            if ay < ey || (ay == ey && ad < ed) {
                break; // past the window: the rest is later
            }
            away = away.wrapping_add(read_u8(e + 8));
        }
        if away as i8 > 2 { // sx15_6: more than two players away
            clubs.push(i);
        }
    }
    clubs
}

/// Moves the fixtures of the clubs that would play near `date` with more than two players away on international duty.
///
/// The window is the day before and the day after (`exe::date_sub_days`, `exe::date_add_days`). Without the club marks
/// (`national_teams_fields::CLUB_MARKS`) they are built for the date, the day before and the day after
/// (`exe::nationalteams_mark_clubs_with_fixtures`). Then each club of `clubs_with_clashing_call_ups` has its fixture
/// near the date rescheduled by the fixture manager (`FIXTURE_MANAGER::reschedule_club_fixtures_near_date` on
/// `G_FIXTURE_MANAGER`, with the club record and the date).
/// Original: `NationalTeams::reschedule_clashing_club_fixtures` (0x7777e0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reschedule_clashing_club_fixtures(this: usize, date: usize) {
    let (mut before, mut after) = ([0u8; 8], [0u8; 8]);
    exe::date_sub_days(date, before.as_mut_ptr() as usize, 1);
    exe::date_add_days(date, after.as_mut_ptr() as usize, 1);
    if read_usize(this + national_teams_fields::CLUB_MARKS) == 0 {
        covered!("sx15_6 marks the clubs of three days");
        exe::nationalteams_mark_clubs_with_fixtures(this, date);
        let (mut day, mut next, mut copy) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        exe::date_sub_days(date, day.as_mut_ptr() as usize, 1);
        exe::nationalteams_mark_clubs_with_fixtures(this, day.as_mut_ptr() as usize);
        let r = exe::date_add_days(day.as_mut_ptr() as usize, next.as_mut_ptr() as usize, 2);
        exe::date_operator_3d(day.as_mut_ptr() as usize, copy.as_mut_ptr() as usize, r);
        exe::nationalteams_mark_clubs_with_fixtures(this, day.as_mut_ptr() as usize);
    }
    let reschedule: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE);
    for i in clubs_with_clashing_call_ups(this, &before, &after) {
        covered!("sx15_6 club fixture moved");
        reschedule(read_usize(G_FIXTURE_MANAGER), club_at(i as isize).addr(), date);
    }
}

/// `FIXTURE_MANAGER::reschedule_club_fixtures_near_date` (thiscall(fixture manager, club, date), ret 8).
const FN_FIXTURE_MANAGER_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE: usize = 0x5abb10;

/// Comparer for `reschedule_clashing_club_fixtures`: it builds the club marks and moves fixtures (it cannot run twice),
/// so only the original runs; the plan checked afterwards: the club marks exist (the routine builds them when missing).
pub(crate) unsafe extern "thiscall" fn compare_reschedule_clashing_club_fixtures(this: usize, date: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, date);
    let marks = read_usize(this + national_teams_fields::CLUB_MARKS);
    SCOREBOARD.record(marks != 0, || format!("reschedule_clashing_club_fixtures(day {}): no club marks", read_i16(date)));
}
