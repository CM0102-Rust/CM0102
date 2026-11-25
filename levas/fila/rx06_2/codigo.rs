// --- rx06_2: the scout list's targets (player_search.cpp): PLAYER_SEARCH::build_scout_targets (0x7b0050); uses the
// result list helpers of sx17_2 and sx17_5 ---

/// The ten marks the scout list starts with on a new assignment (+0x1c6, ten dwords set to `i32::MAX`) [inference: the
/// best-so-far marks of the scout's days].
const SCOUT_TARGET_MARKS: usize = 0x1c6;
/// The scout list's orders by rating, passed to qsort by address: current (+0x21) then potential (+0x25), highest first.
const ORDER_BY_CURRENT_RATING: usize = 0x7bce80;
const ORDER_BY_POTENTIAL_RATING: usize = 0x7bceb0;
/// The role codes `match_man_score_player_for_role` rates a player with: his current and his potential worth.
const ROLE_CURRENT_WORTH: u32 = 0x1e82;
const ROLE_POTENTIAL_WORTH: u32 = 0x1f00;
/// The result fields the scout list fills: current and potential worth (dwords), their grades (0..5, bytes), the byte
/// the original leaves unset and the owner's note rating.
const RESULT_CURRENT_WORTH: usize = 0x21;
const RESULT_POTENTIAL_WORTH: usize = 0x25;
const RESULT_CURRENT_GRADE: usize = 0x29;
const RESULT_POTENTIAL_GRADE: usize = 0x2a;
const RESULT_UNSET_BYTE: usize = 0x2b;
const RESULT_NOTE_RATING: usize = 0x2c;

/// The opponent of the scout's club in its first unplayed fixture (home goals 0xff) from tomorrow to 8 weeks ahead
/// (`fixture_manager_find_fixtures` with every competition, side −2 and the reserves); 0 when there is none.
unsafe fn scout_next_opponent(scout_club: Club) -> Club {
    let mut list = [0u8; 0xe];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let (mut tomorrow, mut later) = ([0u8; 8], [0u8; 8]);
    exe::date_add_days(G_CURRENT_DATE, tomorrow.as_mut_ptr() as usize, 1);
    exe::date_add_weeks(G_CURRENT_DATE, &mut later, 8);
    let (tomorrow, later) = (tomorrow.as_ptr() as usize, later.as_ptr() as usize);
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(tomorrow), read_u32(tomorrow + 2), read_u16(later) as u32,
                                       read_u16(later + 2) as u32, read_u32(scout_club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    let mut opponent = Club(0);
    if read_u16(list + llist::COUNT) != 0 {
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let mut fixture = Fixture(exe::llist_get_next(list));
        while !fixture.is_null() && fixture.home_goals() != -1 {
            fixture = Fixture(exe::llist_get_next(list));
        }
        if !fixture.is_null() {
            opponent = if fixture.home_club() == scout_club { fixture.away_club() } else { fixture.home_club() };
        }
    }
    exe::drop_llist(list);
    opponent
}

/// What the scout makes of a player in a role: `match_man_score_player_for_role(player, sheet, 0, 0, role, 0, 0, scout,
/// −1, 0 × 6)`.
pub(crate) unsafe fn scout_role_score(player: Staff, scout: Staff, role: u32) -> i32 {
    let score: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    score(player.addr(), player.player().addr(), 0, 0, role, 0, 0, scout.addr() as u32, u32::MAX, 0, 0, 0, 0, 0, 0)
}

/// The two byte sums a scout weighs a player sheet by: speed (pace + acceleration) and physique (work rate + teamwork +
/// strength + stamina + jumping), both wrapping as bytes.
pub(crate) unsafe fn scout_speed_and_physique(sheet: PlayerSheet) -> (i8, i8) {
    let speed = sheet.player_pace().wrapping_add(sheet.acceleration());
    let physique = sheet.work_rate().wrapping_add(sheet.teamwork()).wrapping_add(sheet.strength()).wrapping_add(sheet.stamina())
        .wrapping_add(sheet.jumping());
    (speed, physique)
}

/// `value` × `factor` (an f64 of the exe) through `_ftol`.
unsafe fn scout_scaled(value: i32, factor: usize) -> i32 {
    ftol(value as f64 * read_f64(factor))
}

/// The bonus of the athletes: with physique above 75 and speed above 30, × 1.2; above 65 and 24, × 1.1; above 50 and
/// 24, × 1.05.
unsafe fn scout_athlete_bonus(worth: i32, physique: i8, speed: i8) -> i32 {
    if physique > 75 && speed > 30 {
        scout_scaled(worth, rdata::F64_1_2)
    } else if physique > 65 && speed > 24 {
        scout_scaled(worth, rdata::F64_1_1)
    } else if physique > 50 && speed > 24 {
        scout_scaled(worth, rdata::F64_1_05)
    } else {
        worth
    }
}

/// The corrections of a player's current worth: the athlete bonus, × 0.75 with speed below 15 (× 0.95 below 20), × 0.85
/// with physique below 25 (× 0.95 below 40) and × 0.85 over 34 years.
pub(crate) unsafe fn scout_current_discounts(mut worth: i32, age: i8, physique: i8, speed: i8) -> i32 {
    worth = scout_athlete_bonus(worth, physique, speed);
    if speed < 15 {
        worth = scout_scaled(worth, rdata::F64_0_75);
    } else if speed < 20 {
        worth = scout_scaled(worth, rdata::F64_0_95);
    }
    if physique < 25 {
        worth = scout_scaled(worth, rdata::F64_0_85);
    } else if physique < 40 {
        worth = scout_scaled(worth, rdata::F64_0_95);
    }
    if age > 34 {
        worth = scout_scaled(worth, rdata::F64_0_85); // rx06_2: the veteran's discount
    }
    worth
}

/// The corrections of a player's future worth: with `low_current`, × 0.85 when the current worth is below 5250 (× 0.9
/// below 7000 when over 19); × 0.75 with speed below 15 (× 0.95 below 24), × 0.75 with physique below 30 (× 0.9 below
/// 45) and the athlete bonus.
pub(crate) unsafe fn scout_potential_discounts(mut worth: i32, current: i32, age: i8, physique: i8, speed: i8, low_current: bool) -> i32 {
    if low_current {
        if current < 5250 {
            worth = scout_scaled(worth, rdata::F64_0_85);
        } else if current < 7000 && age > 19 {
            worth = scout_scaled(worth, rdata::F64_0_9);
        }
    }
    if speed < 15 {
        worth = scout_scaled(worth, rdata::F64_0_75);
    } else if speed < 24 {
        worth = scout_scaled(worth, rdata::F64_0_95);
    }
    if physique < 30 {
        worth = scout_scaled(worth, rdata::F64_0_75);
    } else if physique < 45 {
        worth = scout_scaled(worth, rdata::F64_0_9);
    }
    scout_athlete_bonus(worth, physique, speed)
}

/// The scout list's two worths of a player: now, random(500) − random(500) + the role score `ROLE_CURRENT_WORTH`,
/// corrected (`scout_current_discounts`); under 25, the future one with `ROLE_POTENTIAL_WORTH` (on a youth assignment
/// first (worth + 100 × potential ability) × 2 / 3 and the low current correction), corrected
/// (`scout_potential_discounts`); 0 from 25 on.
unsafe fn scout_target_worths(this: usize, player: Staff, scout: Staff) -> (i32, i32) {
    let (speed, physique) = scout_speed_and_physique(player.player());
    let age = read_i8(player.addr() + staff::AGE);
    let luck = exe::utils_random(500);
    let luck = luck.wrapping_sub(exe::utils_random(500));
    let current = scout_current_discounts(luck.wrapping_add(scout_role_score(player, scout, ROLE_CURRENT_WORTH)), age, physique, speed);
    if age >= 25 {
        return (current, 0);
    }
    let luck = exe::utils_random(500);
    let luck = luck.wrapping_sub(exe::utils_random(500));
    let mut worth = luck.wrapping_add(scout_role_score(player, scout, ROLE_POTENTIAL_WORTH));
    let youth = read_u32(this + player_search_fields::ASSIGNED_BASE) != 0;
    if youth {
        worth = worth.wrapping_add(player.player().potential_ability() as i32 * 100).wrapping_mul(2) / 3;
    }
    (current, scout_potential_discounts(worth, current, age, physique, speed, youth))
}

/// The grade (0..5) of a worth against the scout club's target: 5 above 1.2 × target and 2 × club reputation + 12750;
/// 4 above the target and 2 × reputation + `four_bar`; 3, 2, 1 above 0.8, 0.7, 0.6 × target; else 0.
pub(crate) unsafe fn scout_grade(worth: i32, target: f64, reputation: i32, four_bar: i32) -> u8 {
    let w = worth as f64;
    if w > target * read_f64(rdata::F64_1_2) && worth > reputation * 2 + 12750 {
        5
    } else if w > target && worth > reputation * 2 + four_bar {
        4
    } else if w > target * read_f64(rdata::F64_0_8) {
        3
    } else if w > target * read_f64(rdata::F64_0_7) {
        2
    } else if w > target * read_f64(rdata::F64_0_6) {
        1
    } else {
        0
    }
}

/// Whether a person may be on the scout list at all: a player (job player, player-manager, player-assistant or
/// player-coach) whose contract club is not the scout's, with a player sheet, not retiring, without a future transfer
/// and, when following an opponent, at that club.
unsafe fn scout_target_candidate(person: Staff, scout_club: Club, opponent: Club) -> bool {
    let club = player_search_contract_club(person);
    if !matches!(person.job_for_club(), job_kind::PLAYER | job_kind::PLAYER_COACH | job_kind::PLAYER_MANAGER | job_kind::PLAYER_ASSISTANT_MANAGER) {
        return false;
    }
    if club == scout_club.addr() || person.player().is_null() {
        return false;
    }
    if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), person) != 0 {
        return false;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 {
        return false;
    }
    opponent.is_null() || person.club_job() == opponent
}

/// Rates one person for the scout list and appends him when he is worth it: the two worths (`scout_target_worths`; both 0
/// for a youth assignment over 24); without an opponent, only when one of them is above
/// 0.6 × the target and the player's interest in joining the scout's club is at least 3.15. The result: his name ref,
/// the default date, today, flag 1, the worths, their grades (the potential's 4 above 2 × reputation + 10500, the
/// current's above + 10750), the unset byte 0 and the owner's note rating.
unsafe fn scout_target_add(this: usize, person: Staff, scout: Staff, scout_club: Club, opponent: Club, target: i32, default_date: usize) {
    let (mut current, mut potential) = scout_target_worths(this, person, scout);
    if read_u32(this + player_search_fields::ASSIGNED_BASE) != 0 && read_i8(person.addr() + staff::AGE) > 24 {
        current = 0;
        potential = 0;
    }
    if opponent.is_null() {
        let bar = target as f64 * read_f64(rdata::F64_0_6);
        if current as f64 <= bar && potential as f64 <= bar {
            return;
        }
        let interest = exe::shortlist_manager_staff_interest_in_joining_club(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), person, scout_club, 1, 0, 1, 0);
        if interest < read_f64(rdata::F64_3_15) {
            return; // rx06_2: he would not come
        }
    }
    let mut entry = [0u8; SCOUT_DAY_RESULT_SIZE];
    let e = entry.as_mut_ptr() as usize;
    exe::fill_staff_name_ref(person, e);
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(e + 0x10, tmp.as_mut_ptr() as usize, default_date);
    exe::date_operator_3d(e + search_result::LAST_UPDATE_DATE, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(e + search_result::FLAGS, 1);
    write_i32(e + RESULT_CURRENT_WORTH, current);
    write_i32(e + RESULT_POTENTIAL_WORTH, potential);
    write_u8(e + RESULT_NOTE_RATING, player_search_note_rating(this, person));
    let reputation = scout_club.reputation() as i32;
    write_u8(e + RESULT_CURRENT_GRADE, scout_grade(current, target as f64, reputation, 10750));
    write_u8(e + RESULT_POTENTIAL_GRADE, scout_grade(potential, target as f64, reputation, 10500));
    write_u8(e + RESULT_UNSET_BYTE, 0); // the original leaves its stack leftover here
    player_search_append_result(this, e, 0x60c);
}

/// Whether result `r` stays on a scout list without an opponent: the player's nation (his club's, else his own) is the
/// assigned nation or its region the assigned region; or, on a youth assignment, a player under 20 from the region of
/// the scout club's nation or worth more than 33333 later, who is free or at least 3.5 interested in the scout's club.
/// Never one of the scout club's nation whom the scout already rates at 4000 or more. It is weighed for every result,
/// also when following an opponent (then nothing is removed).
unsafe fn scout_target_keeps(this: usize, r: usize, scout: Staff, scout_club: Club) -> bool {
    let person = staff_at(read_i32(r + 0xc) as isize);
    let club = person.club_job();
    let mut nation = if !club.is_null() { club.nation() } else { Nation(0) };
    if nation.is_null() {
        nation = person.nation();
    }
    let assigned = read_i32(this + player_search_fields::ASSIGNED_NATION);
    let region = read_i8(this + player_search_fields::ASSIGNED_REGION);
    let keep = if assigned > -1 && !nation.is_null() && read_i32(nation.addr()) == assigned {
        true
    } else if region > -1 && !nation.is_null() && nation.actual_region() as i8 == region {
        true
    } else if read_u32(this + player_search_fields::ASSIGNED_BASE) != 0 && read_i8(person.addr() + staff::AGE) < 20 {
        let near = !nation.is_null() && nation.actual_region() == scout_club.nation().actual_region();
        (near || read_i32(r + RESULT_POTENTIAL_WORTH) > 33333)
            && (club.is_null()
                || exe::shortlist_manager_staff_interest_in_joining_club(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), person, scout_club, 0, 0, 5, 0) >= read_f64(rdata::F64_3_5))
    } else {
        false
    };
    keep && !(nation == scout_club.nation() && exe::database_get_staff_reputation_for_viewer(person, scout, 0) >= 4000)
}

/// Fills a scout list with the players its assignment points to (the list's scout through `get_scout`; without one, or
/// without a club, the player_search.cpp box of line 0x4bc / 0x4c6 and 0). The target is the scout club's
/// (`player_search_get_club_target_rating`) held between 1750 and 35000. The old results are freed, capacity, count and
/// `SCOUT_DAY_REPORT_MARK` zeroed and the ten marks (`SCOUT_TARGET_MARKS`) set to `i32::MAX`. Following the next
/// opposition (`ASSIGNED_OPPONENT`), the opponent is that of the scout club's first unplayed fixture in the next 8 weeks
/// (`scout_next_opponent`; none: 0) and becomes `SCOUT_DAY_TARGET_CLUB`. Every person who may be a target
/// (`scout_target_candidate`) is rated and appended (`scout_target_add`). Then the results are sorted by current and by
/// potential worth; without an opponent, those the assignment does not keep (`scout_target_keeps`) are replaced by the
/// last one. Finally the list is cut at random to 50 + random(current ability of the scout / 3 + 2 × his
/// professionalism + 20), each cut replacing a random result by the last; what is left is sorted by person, or the
/// array is freed. Returns 1.
/// Original: `PLAYER_SEARCH::build_scout_targets` (0x7b0050, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_build_scout_targets(this: usize) -> u32 {
    let mut default_date = [0u8; 8];
    exe::date_date_54a110(default_date.as_mut_ptr() as usize);
    let scout = Staff(get_scout(this));
    if scout.is_null() {
        show_error_at(0xa701b0, 0x4bc);
        return 0;
    }
    let scout_club = scout.club_job();
    if scout_club.is_null() {
        show_error_at(0xa701b0, 0x4c6);
        return 0;
    }
    let target = player_search_get_club_target_rating(scout_club.addr(), core::ptr::null_mut(), core::ptr::null()).clamp(1750, 35000);
    if read_usize(this + player_search_fields::RESULTS) != 0 {
        exe::crt_free(read_usize(this + player_search_fields::RESULTS));
        write_usize(this + player_search_fields::RESULTS, 0);
    }
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    for k in 0..10 {
        write_i32(this + SCOUT_TARGET_MARKS + 4 * k, i32::MAX);
    }
    let mut opponent = Club(0);
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) != 0 {
        opponent = scout_next_opponent(scout_club);
        if opponent.is_null() {
            return 0;
        }
        covered!("rx06_2 the next opponent");
        write_u32(this + SCOUT_DAY_TARGET_CLUB, read_u32(opponent.addr()));
    }
    let mut i = 0;
    while i < read_i32(G_STAFF_COUNT) {
        let person = staff_at(i as isize);
        if scout_target_candidate(person, scout_club, opponent) {
            scout_target_add(this, person, scout, scout_club, opponent, target, default_date.as_ptr() as usize);
        }
        i += 1;
    }
    let count = || read_i32(this + player_search_fields::RESULT_COUNT);
    if count() == 0 {
        return 1;
    }
    let results = read_usize(this + player_search_fields::RESULTS);
    exe::crt_qsort(results, count() as u32, SCOUT_DAY_RESULT_SIZE as u32, ORDER_BY_CURRENT_RATING);
    exe::crt_qsort(results, count() as u32, SCOUT_DAY_RESULT_SIZE as u32, ORDER_BY_POTENTIAL_RATING);
    let at = |k: i32| read_usize(this + player_search_fields::RESULTS).wrapping_add((k as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    let mut k = 0;
    while k < count() {
        // the original weighs every result, and removes only without an opponent
        let keeps = scout_target_keeps(this, at(k), scout, scout_club);
        if opponent.is_null() && !keeps {
            scout_day_remove_result(this, at(k));
            k -= 1;
        }
        k += 1;
    }
    let range = (scout.non_player().current_ability() as i32 / 3) + (scout.professionalism() as i32) * 2 + 20;
    let keep = exe::utils_random(range) + 50; // rx06_2: how many the scout keeps
    while count() > keep {
        let j = exe::utils_random(count());
        scout_day_remove_result(this, at(j));
    }
    player_search_sort_or_free_results(this);
    1
}

/// Comparer for `PLAYER_SEARCH::build_scout_targets` (model `compare_run_scout_day`): it frees the old results,
/// allocates and draws, so the copy runs first as a dry run on a duplicate of the old array; its list object, result
/// array, random generator and contract lookup are kept, the state goes back and the original runs. Compared: the
/// return, the object (all but the array pointer), the result array (each result's unset byte +0x2b ignored, the
/// original's stack leftover), the generator and the contract lookup.
pub(crate) unsafe extern "thiscall" fn compare_player_search_build_scout_targets(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let mut regions: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    regions.push((this, SCOUT_DAY_OBJECT_SIZE));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let old = read_usize(this + player_search_fields::RESULTS);
    if old != 0 {
        let n = read_i32(this + SCOUT_DAY_RESULT_CAPACITY).max(1) as usize * SCOUT_DAY_RESULT_SIZE;
        let copy = exe::crt_malloc(n as u32);
        core::ptr::copy_nonoverlapping(old as *const u8, copy as *mut u8, n);
        write_usize(this + player_search_fields::RESULTS, copy);
    }
    let rd = crate::rng::pause_trace(|| crate::rng::without_counting(|| player_search_build_scout_targets(this)));
    let array = |this: usize| -> Vec<u8> {
        let a = read_usize(this + player_search_fields::RESULTS);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let mut v = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        for entry in v.chunks_exact_mut(SCOUT_DAY_RESULT_SIZE) {
            entry[RESULT_UNSET_BYTE] = 0;
        }
        v
    };
    let mut dry_state = crate::comparison::MemorySnapshot::read(&regions);
    let dry_results = array(this);
    let dry_array = read_usize(this + player_search_fields::RESULTS);
    if dry_array != 0 && dry_array != old {
        exe::crt_free(dry_array);
    }
    before.write_back();
    let r = o(this);
    let mut after = crate::comparison::MemorySnapshot::read(&regions);
    let pointer = this + player_search_fields::RESULTS - this;
    for snapshot in [&mut after, &mut dry_state] {
        if let Some((_, object)) = snapshot.0.last_mut() {
            object[pointer..pointer + 4].fill(0);
        }
    }
    let results = array(this);
    let same = r == rd && after == dry_state && results == dry_results;
    SCOREBOARD.record(same, || format!("build_scout_targets(search {:#x}) original={} rust={} results {} x {} {:?}", this, r, rd,
        results.len() / SCOUT_DAY_RESULT_SIZE, dry_results.len() / SCOUT_DAY_RESULT_SIZE, first_differences(&after, &dry_state)));
    r
}
