// ---------- sx04c_24: the team of the week of a competition (award/nation_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\nation_awards.cpp" (the error box of the team of the week).
const NATION_AWARDS_SOURCE_FILE: usize = 0xa5b26c;
/// A team of the week candidate (11 bytes): the person id (+0), the club pointer (+4), the match grade (+8) and the
/// position mask (+9, word); the buffer grows by 200 candidates.
const WEEK_CANDIDATE_SIZE: usize = 11;
const WEEK_CANDIDATES_STEP: usize = 200;
/// The players a team of the week needs before it stops going back a week (more than 20).
const WEEK_CANDIDATES_ENOUGH: usize = 20;
/// The qsort order of the candidates: the higher match grade first (`nation_awards_compare_byte_8_descending`).
const FN_WEEK_CANDIDATES_ORDER: usize = 0x76a460;
/// A match record as `Match_Stats::read_match_record` fills it: 0x11f bytes, then the two team blocks
/// (`match_record::SIDE_SIZE` each), built and destroyed by the game's team block constructor and destructor.
const MATCH_RECORD_BYTES: usize = match_record::HOME_TEAM + 2 * match_record::SIDE_SIZE;
const MATCH_TEAM_BLOCK_CONSTRUCTOR: usize = 0x453c30;

/// How many times the hooked team of the week ran (counted by its comparer): the nations' `update_team_of_the_week`
/// comparers check from it whether the original built a team.
pub(crate) static TEAM_OF_THE_WEEK_BUILDS: AtomicU32 = AtomicU32::new(0);

/// Whether date `a` comes before date `b` as the original compares them: the year words unsigned, then the day words
/// signed.
unsafe fn week_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + cm_date::YEAR), read_u16(b + cm_date::YEAR));
    ya < yb || ya == yb && read_i16(a) < read_i16(b)
}

/// The candidates' buffer: the pointer (`exe::crt_malloc`/`exe::crt_realloc`, 0 none), how many it holds and its
/// capacity.
struct WeekCandidates {
    buf: usize,
    count: usize,
    capacity: usize,
}

/// Adds the players of one side of a played match to the candidates: the starting eleven and the substitutes who came
/// on (`match_record_row::MINUTE_ON` not 0xff), each with a staff id and a player id (not −1) and a staff id not above
/// the staff count (`G_STAFF_COUNT`, compared signed: an id equal to the count gets in, defect of the original). Each
/// takes the side's club (`fixture::HOME_CLUB` or `AWAY_CLUB`), his match grade and his position: the formation slot's
/// word for the eleven (`match_record_team::FORMATION_X`, read as the slot's position mask), the natural position
/// mask for a substitute with a player record (`exe::staff_get_natural_position_mask`, else 0). The buffer grows by
/// 200; out of memory, the nation_awards.cpp 1960 box and `false` (the old buffer is lost, as in the original).
unsafe fn week_add_side(c: &mut WeekCandidates, record: usize, fixture: usize, side: usize) -> bool {
    let team = record + match_record::HOME_TEAM + side * match_record::SIDE_SIZE;
    let club = read_usize(fixture + if side == 0 { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
    for slot in 0..20 {
        let row = record + match_record::ROW_STAFF_ID + side * match_record::SIDE_SIZE + slot * match_record_row::SIZE;
        if slot > 10 && read_u8(row + match_record_row::MINUTE_ON) == 0xff {
            continue;
        }
        let id = read_i32(row + match_record_row::STAFF_ID);
        if id == -1 || read_i32(row + match_record_row::PLAYER_ID) == -1 || id > read_i32(G_STAFF_COUNT) {
            continue;
        }
        if c.count == c.capacity {
            c.capacity += WEEK_CANDIDATES_STEP;
            let bytes = (c.capacity * WEEK_CANDIDATE_SIZE) as u32;
            c.buf = if c.buf != 0 { exe::crt_realloc(c.buf, bytes) } else { exe::crt_malloc(bytes) };
            if c.buf == 0 {
                show_error_at(NATION_AWARDS_SOURCE_FILE, 0x7a8);
                return false;
            }
        }
        let entry = c.buf + c.count * WEEK_CANDIDATE_SIZE;
        write_i32(entry, id);
        write_usize(entry + 4, club);
        write_u8(entry + 8, read_u8(row + match_record_row::MATCH_GRADE));
        let position = if slot < 11 {
            read_u16(team + match_record_team::FORMATION_X + 2 * slot)
        } else {
            let person = staff_at(id as isize);
            if read_u32(person.addr() + staff::PLAYER) != 0 { exe::staff_get_natural_position_mask(person.addr()) as u16 } else { 0 }
        };
        write_u16(entry + 9, position);
        c.count += 1;
    }
    true
}

/// What the matches of one week gave: the candidates were added (or there were not enough fixtures), a fixture is
/// not played yet (the candidates are dropped), or memory ran out.
enum WeekOutcome {
    Added,
    Unplayed,
    OutOfMemory,
}

/// The matches of the week in `list` (already filled): for each fixture, its match record (`fixture::MATCH_STATS_SLOT`
/// −1 or a record that cannot be read: `WeekOutcome::Unplayed`); the first fixture's date becomes `first_day` and
/// `last_day`, a later fixture on another day becomes `last_day`; then both sides' players
/// (`week_add_side`).
unsafe fn week_add_matches(c: &mut WeekCandidates, list: usize, record: usize, first_day: usize, last_day: usize) -> WeekOutcome {
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut fixture = exe::llist_get_next(list);
    let mut k = 0;
    while fixture != 0 {
        let slot = read_i32(fixture + fixture::MATCH_STATS_SLOT);
        if slot == -1 || exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), slot as u32, record) == 0 {
            return WeekOutcome::Unplayed;
        }
        let (day, year) = (read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
        if k == 0 {
            exe::new_date(first_day, day, year);
            copy_bytes(first_day, last_day, 8);
        }
        if day as i32 != read_i16(last_day) as i32 {
            exe::new_date(last_day, day, year);
        }
        for side in 0..2 {
            if !week_add_side(c, record, fixture, side) {
                return WeekOutcome::OutOfMemory;
            }
        }
        k += 1;
        fixture = exe::llist_get_next(list);
    }
    WeekOutcome::Added
}

/// Copies `n` bytes from `from` to `to` (the original's `Date::operator=`).
unsafe fn copy_bytes(from: usize, to: usize, n: usize) {
    for i in 0..n {
        write_u8(to + i, read_u8(from + i));
    }
}

/// Offers the sorted candidates to the award: the selection cleared (`team_award_clear_selection`), the substitutes
/// (`team_award_obj::SUBSTITUTES`) those its club competition names (`League::substitutes_named`, 3 without one), each
/// candidate in order (`team_award_add_candidate` with the person, the club, the grade as a float and the position;
/// all of them, whatever the award answers) and the period dates (`team_award_set_dates`: the first and last match
/// days and the week's date).
unsafe fn week_offer_candidates(award: usize, c: &WeekCandidates, first_day: usize, last_day: usize, week: usize) {
    exe::crt_qsort(c.buf, c.count as u32, WEEK_CANDIDATE_SIZE as u32, FN_WEEK_CANDIDATES_ORDER);
    team_award_clear_selection(award);
    let mut substitutes = 3u8;
    let table = read_usize(G_COMP_OBJECT_TABLE);
    let comp_id = read_usize(award + award_base::CLUB_COMP);
    if table != 0 && comp_id != 0 {
        let comp = League(read_usize(table + read_u32(comp_id) as usize * 4));
        if !comp.is_null() {
            substitutes = comp.substitutes_named();
        }
    }
    write_u8(award + team_award_obj::SUBSTITUTES, substitutes);
    for k in 0..c.count {
        let entry = c.buf + k * WEEK_CANDIDATE_SIZE;
        team_award_add_candidate(award, staff_at(read_i32(entry) as isize).addr(), read_usize(entry + 4), read_u8(entry + 8) as f32, read_u16(entry + 9));
    }
    team_award_set_dates(award, first_day, last_day, week);
}

/// Builds the team of the week of a competition into a team award.
///
/// Goes back week by week from today's nearest weekday 0 (`exe::date_get_nearest_weekday`) while the week is after
/// the later of 100 days ago and the award's last update (`team_award_obj::UPDATE_DATE`): the competition's fixtures
/// from three days before the week's date to it (`exe::fixture_manager_find_fixtures`); with two or more, their
/// players (`week_add_matches`); a fixture not played yet drops the candidates gathered so far. It stops once more than
/// 20 candidates are in. With none, nothing (defect of the original: a buffer allocated before the drop is not freed).
/// Otherwise the candidates sorted by grade go to the award (`week_offer_candidates`) and the buffer is freed. The
/// match record and the fixture list are the original's stack objects (built and destroyed by the game's
/// constructors).
/// Original: `nation_awards_build_team_of_the_week` (0x769c70, nation_awards.cpp, thiscall, ret 8; `this` unused).
pub(crate) unsafe extern "thiscall" fn nation_awards_build_team_of_the_week(_this: usize, comp_id: u32, award: usize) {
    let (mut start, mut week, mut week_start) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (mut first_day, mut last_day) = ([0u8; 8], [0u8; 8]);
    let (start, week, week_start) = (start.as_mut_ptr() as usize, week.as_mut_ptr() as usize, week_start.as_mut_ptr() as usize);
    let (first_day, last_day) = (first_day.as_mut_ptr() as usize, last_day.as_mut_ptr() as usize);
    exe::date_sub_days(G_CURRENT_DATE, start, 100);
    copy_bytes(G_CURRENT_DATE, week, 8);
    exe::date_date_54a110(week_start);
    exe::date_date_54a110(first_day);
    exe::date_date_54a110(last_day);
    let mut list = [0u8; 14];
    let list = list.as_mut_ptr() as usize;
    let llist_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    llist_new(list);
    let mut record = [0u8; MATCH_RECORD_BYTES];
    let record = record.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(record + match_record::HOME_TEAM, match_record::SIDE_SIZE as u32, 2, MATCH_TEAM_BLOCK_CONSTRUCTOR, MATCH_TEAM_BLOCK_DESTRUCTOR);
    let mut c = WeekCandidates { buf: 0, count: 0, capacity: 0 };
    if week_date_before(start, award + team_award_obj::UPDATE_DATE) {
        copy_bytes(award + team_award_obj::UPDATE_DATE, start, 8);
    }
    exe::date_get_nearest_weekday(week, 0, week);
    let mut memory = true;
    while week_date_before(start, week) {
        covered!("sx04c_24 team of the week: a week");
        exe::date_sub_days(week, week_start, 3);
        let n = exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(week_start), read_u32(week_start + 2), read_u32(week),
            read_u32(week + 2), u32::MAX, list, comp_id, 0xffff_fffe, u32::MAX);
        if n > 1 {
            match week_add_matches(&mut c, list, record, first_day, last_day) {
                WeekOutcome::Added => {}
                WeekOutcome::Unplayed => c.count = 0,
                WeekOutcome::OutOfMemory => {
                    memory = false;
                    break;
                }
            }
            if c.count > WEEK_CANDIDATES_ENOUGH {
                break;
            }
        }
        exe::llist_clear(list);
        let mut previous = [0u8; 8];
        exe::date_sub_weeks(week, &mut previous, 1);
        copy_bytes(previous.as_ptr() as usize, week, 8);
    }
    if memory && c.count != 0 {
        covered!("sx04c_24 team of the week built");
        week_offer_candidates(award, &c, first_day, last_day, week);
        if c.buf != 0 {
            exe::crt_free(c.buf);
        }
    }
    exe::crt_vector_destructor_iterator(record + match_record::HOME_TEAM, match_record::SIDE_SIZE as u32, 2, MATCH_TEAM_BLOCK_DESTRUCTOR);
    let llist_drop: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    llist_drop(list);
}

/// Comparer for `nation_awards_build_team_of_the_week`: side by side over the team award (`team_award_obj::SIZE`
/// bytes: the selection, the substitutes and the dates); it counts the builds for the nations' comparers
/// (`TEAM_OF_THE_WEEK_BUILDS`).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_build_team_of_the_week(this: usize, comp_id: u32, award: usize) {
    TEAM_OF_THE_WEEK_BUILDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I0%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(award, team_award_obj::SIZE as usize)], || o(this, comp_id, award),
        || nation_awards_build_team_of_the_week(this, comp_id, award));
    SCOREBOARD.record(equal, || format!("nation_awards_build_team_of_the_week(comp {}, award {:#x}) {:?}", comp_id, award, first_differences(&a, &b)));
}
