// --- sx17_4: the scout list's assignment and the watched player's match history (player_search.cpp):
// start_scout_assignment (0x7ae9c0), clear_scout_assignment (0x7aee80), player_search_slot_sum_assists (0x7bd8c0),
// player_search_slot_sum_goals_conceded (0x7bd910) and player_search_slot_average_rating (0x7bd960) ---

/// `PLAYER_SEARCH::build_scout_targets` (0x7b0050, thiscall): fills a scout list with the players of its assignment.
const FN_PLAYER_SEARCH_BUILD_SCOUT_TARGETS: usize = 0x7b0050;

/// What a scout list is set to watch (`start_scout_assignment`, in the order it checks the arguments).
#[derive(Clone, Copy, PartialEq, Debug)]
enum ScoutAssignment {
    Nation(i32),
    /// The region argument as passed (a dword; its low byte is the region).
    Region(u32),
    Competition(i32),
    Youth(u32),
    NextOpposition(u32),
}

/// The assignment the arguments ask for: a nation (a valid index), else a region (1..14), else a club competition (a
/// valid index), else youth players (non-zero), else the next opposition (non-zero); none when nothing is valid.
unsafe fn player_search_pick_assignment(nation: u32, region: u32, comp: u32, youth: u32, opponent: u32) -> Option<ScoutAssignment> {
    let (nation, comp) = (nation as i32, comp as i32);
    if nation >= 0 && nation < read_i32(G_NATION_COUNT) {
        Some(ScoutAssignment::Nation(nation))
    } else if (1..=14).contains(&(region as u8 as i8)) {
        Some(ScoutAssignment::Region(region))
    } else if comp >= 0 && comp < read_i32(G_CLUB_COMP_COUNT) {
        Some(ScoutAssignment::Competition(comp))
    } else if youth != 0 {
        Some(ScoutAssignment::Youth(youth))
    } else if opponent != 0 {
        Some(ScoutAssignment::NextOpposition(opponent))
    } else {
        None
    }
}

/// Writes the list's title (+0xb) for the assignment: "{}<%s - nationality>{} search" with the nation's adjective
/// (`nation_get_nationality_adjective`, 200 bytes, style 1) and gender, "{}<%s - regionality>{} search" with the region's
/// name (`get_region_name`) and gender 0x64, "{}<%s - competition>{} search" with the competition's name and gender,
/// "Youth search" or "Next Opposition" (`langlib_sprintf`).
unsafe fn player_search_write_assignment_title(title: usize, assignment: ScoutAssignment) {
    let mut name = [0u8; 0xc8];
    let n = name.as_mut_ptr() as usize;
    match assignment {
        ScoutAssignment::Nation(k) => {
            let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add(k as usize * nation::SIZE));
            exe::nation_get_nationality_adjective(nation, n, 0xc8, 1);
            let gender = nation.gender_name() as i8 as i32;
            story_lang!(title, 0xa70240, gender, gender, n); // "{}<%s - nationality>{} search"
        }
        ScoutAssignment::Region(r) => {
            exe::get_region_name(r, n, 0xc8, 1);
            story_lang!(title, 0xa70220, 0x64, 0x64, n); // "{}<%s - regionality>{} search"
        }
        ScoutAssignment::Competition(k) => {
            let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add(k as usize * 0x6b);
            praise_strcpy(n, comp + competition::NAME);
            let gender = read_i8(comp + competition::GENDER_NAME) as i32;
            story_lang!(title, 0xa70200, gender, gender, n); // "{}<%s - competition>{} search"
        }
        ScoutAssignment::Youth(_) => story_lang!(title, 0xa701f0), // "Youth search"
        ScoutAssignment::NextOpposition(_) => story_lang!(title, 0xa701e0), // "Next Opposition"
    }
}

/// The scouting end of an assignment that runs for a few days: today plus 1..3 days (`utils_random(3)` + 1).
unsafe fn player_search_end_in_a_few_days(this: usize) {
    let days = exe::utils_random(3) + 1; // sx17_4: the days of a nation, region, competition or youth assignment
    let mut date = [0u8; 8];
    let end = exe::date_add_days(G_CURRENT_DATE, date.as_mut_ptr() as usize, days as u32);
    let mut copy = [0u8; 8];
    exe::date_operator_3d(this + player_search_fields::SCOUTING_END_DATE, copy.as_mut_ptr() as usize, end);
}

/// A competition assignment watches its season: the first match date into +0x1b6 and the last into the competition
/// date (`Comp::build_match_date_list`, 20-byte entries with the day and year at +0xc/+0xe, freed after); the scouting
/// end is a few days away, or two days before the first match when that is later.
unsafe fn player_search_watch_competition(this: usize, k: i32) {
    let mut count = 0i32;
    let dates = exe::comp_build_match_date_list(read_usize(read_usize(G_COMP_OBJECT_TABLE) + k as usize * 4), &mut count);
    let first = this + 0x1b6;
    let (mut date, mut copy) = ([0u8; 8], [0u8; 8]);
    let d = exe::new_date(date.as_mut_ptr() as usize, read_u16(dates + 0xc) as u32, read_u16(dates + 0xe) as u32);
    exe::date_operator_3d(first, copy.as_mut_ptr() as usize, d);
    let last = dates + count as usize * 0x14;
    let d = exe::new_date(date.as_mut_ptr() as usize, read_u16(last - 8) as u32, read_u16(last - 6) as u32);
    exe::date_operator_3d(this + SCOUT_DAY_COMPETITION_DATE, copy.as_mut_ptr() as usize, d);
    exe::crt_free(dates);
    player_search_end_in_a_few_days(this);
    let end = this + player_search_fields::SCOUTING_END_DATE;
    let before = exe::date_sub_days(first, date.as_mut_ptr() as usize, 2);
    let (year, day) = (read_u16(before + 2), read_i16(before));
    if year > read_u16(end + 2) || (year == read_u16(end + 2) && day > read_i16(end)) {
        covered!("sx17_4 competition starts later");
        let before = exe::date_sub_days(first, date.as_mut_ptr() as usize, 2);
        exe::date_operator_3d(end, copy.as_mut_ptr() as usize, before);
    }
}

/// Sets a scout list to an assignment and builds its targets. The list needs its scout (`get_scout`, else 0 with no
/// box) and must be a scout list (type 4, else the player_search.cpp "Error" box, line 0x18e, and 0). The assignment
/// is the first valid argument (`player_search_pick_assignment`; none: the box, line 0x21d, and 0); the title is
/// written (`player_search_write_assignment_title`) and the nation, region and competition fields set (the others −1).
/// A nation, region or youth assignment ends in 1..3 days, a competition one as `player_search_watch_competition`
/// says; the youth flag (+0x199) or the next opponent (+0x19d) are stored for those; the next opposition ends
/// tomorrow and counts one report already done, the others none. Then the targets are built
/// (`PLAYER_SEARCH::build_scout_targets`) and the SearchFilters record reset to its defaults. Answers 1.
/// Original: `PLAYER_SEARCH::start_scout_assignment` (0x7ae9c0, player_search.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn player_search_start_scout_assignment(this: usize, nation: u32, region: u32, comp: u32, youth: u32, opponent: u32) -> u32 {
    if get_scout(this) == 0 {
        return 0;
    }
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 {
        show_error_box(0xa701b0, 0x18e); // player_search.cpp
        return 0;
    }
    let Some(assignment) = player_search_pick_assignment(nation, region, comp, youth, opponent) else {
        show_error_box(0xa701b0, 0x21d); // player_search.cpp
        return 0;
    };
    player_search_write_assignment_title(this + 0xb, assignment);
    let (n, r, c) = match assignment {
        ScoutAssignment::Nation(k) => (k, -1, -1),
        ScoutAssignment::Region(k) => (-1, k as u8 as i8, -1),
        ScoutAssignment::Competition(k) => (-1, -1, k),
        _ => (-1, -1, -1),
    };
    write_i32(this + player_search_fields::ASSIGNED_NATION, n);
    write_i8(this + player_search_fields::ASSIGNED_REGION, r);
    write_i32(this + player_search_fields::ASSIGNED_COMP, c);
    let mut reports = 0;
    match assignment {
        ScoutAssignment::Competition(k) => player_search_watch_competition(this, k),
        ScoutAssignment::Youth(flag) => {
            write_u32(this + player_search_fields::ASSIGNED_BASE, flag);
            player_search_end_in_a_few_days(this);
        }
        ScoutAssignment::NextOpposition(flag) => {
            covered!("sx17_4 next opposition");
            write_u32(this + player_search_fields::ASSIGNED_OPPONENT, flag);
            let (mut date, mut copy) = ([0u8; 8], [0u8; 8]);
            let end = exe::date_add_days(G_CURRENT_DATE, date.as_mut_ptr() as usize, 1);
            exe::date_operator_3d(this + player_search_fields::SCOUTING_END_DATE, copy.as_mut_ptr() as usize, end);
            reports = 1;
        }
        _ => player_search_end_in_a_few_days(this),
    }
    write_u8(this + SCOUT_DAY_REPORTS_DONE, reports);
    let build: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_PLAYER_SEARCH_BUILD_SCOUT_TARGETS);
    build(this);
    finder_call0(FINDER_FILTERS_SET_DEFAULTS, this + 0x1fa);
    1
}

/// Comparer for `player_search_start_scout_assignment`: it draws the days and builds the targets (allocates and draws),
/// so only the original runs, and the plan is checked: the title the copy writes for the arguments
/// (`player_search_write_assignment_title` into a local buffer), the nation, region and competition fields, the youth
/// or opponent flag, the reports done and, outside the competition, the scouting end 1..3 days away (1 for the next
/// opposition).
pub(crate) unsafe extern "thiscall" fn compare_player_search_start_scout_assignment(this: usize, nation: u32, region: u32, comp: u32, youth: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    let plan = if get_scout(this) != 0 && read_u8(this + player_search_fields::LIST_TYPE) == 4 {
        player_search_pick_assignment(nation, region, comp, youth, opponent)
    } else {
        None
    };
    let r = o(this, nation, region, comp, youth, opponent);
    let Some(assignment) = plan else {
        SCOREBOARD.record(r == 0, || format!("player_search_start_scout_assignment({:#x}) without an assignment returned {}", this, r));
        return r;
    };
    let mut title = [0u8; 0x100];
    player_search_write_assignment_title(title.as_mut_ptr() as usize, assignment);
    let len = c_string_len(title.as_ptr() as usize) + 1;
    let mut ok = r == 1 && core::slice::from_raw_parts((this + 0xb) as *const u8, len) == &title[..len];
    let fields = (read_i32(this + player_search_fields::ASSIGNED_NATION), read_i8(this + player_search_fields::ASSIGNED_REGION),
                  read_i32(this + player_search_fields::ASSIGNED_COMP));
    let end = read_u32(this + player_search_fields::SCOUTING_END_DATE);
    let days = (1..=3).find(|&d| {
        let mut date = [0u8; 8];
        read_u32(exe::date_add_days(G_CURRENT_DATE, date.as_mut_ptr() as usize, d)) == end
    }).unwrap_or(0);
    ok &= match assignment {
        ScoutAssignment::Nation(k) => fields == (k, -1, -1) && (1..=3).contains(&days),
        ScoutAssignment::Region(k) => fields == (-1, k as u8 as i8, -1) && (1..=3).contains(&days),
        ScoutAssignment::Competition(k) => fields == (-1, -1, k),
        ScoutAssignment::Youth(flag) => fields == (-1, -1, -1) && read_u32(this + player_search_fields::ASSIGNED_BASE) == flag && (1..=3).contains(&days),
        ScoutAssignment::NextOpposition(flag) => fields == (-1, -1, -1) && read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == flag && days == 1,
    };
    let reports = read_u8(this + SCOUT_DAY_REPORTS_DONE);
    ok &= reports == matches!(assignment, ScoutAssignment::NextOpposition(_)) as u8;
    SCOREBOARD.record(ok, || format!("player_search_start_scout_assignment({:#x}, {:?}) returned {} fields {:?} days {} reports {}", this, assignment, r, fields, days, reports));
    r
}

/// Ends a scout list's assignment: it must be a scout list (type 4) with its scout (`get_scout`), else the
/// player_search.cpp "Error" box (line 0x237) and 0. The assigned club becomes −1 and its scouted club record leaves the
/// global list and is deleted; the nation, region, competition and target club become −1, the youth and opponent
/// flags 0. Answers 1.
/// Original: `PLAYER_SEARCH::clear_scout_assignment` (0x7aee80, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_clear_scout_assignment(this: usize) -> u32 {
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 || get_scout(this) == 0 {
        show_error_box(0xa701b0, 0x237); // player_search.cpp
        return 0;
    }
    write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if record != 0 {
        exe::llist_remove(G_SCOUT_CLUB_TABLE_LIST, record);
        exe::crt_operator_delete(record);
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    }
    write_i32(this + player_search_fields::ASSIGNED_NATION, -1);
    write_i8(this + player_search_fields::ASSIGNED_REGION, -1);
    write_i32(this + player_search_fields::ASSIGNED_COMP, -1);
    write_u32(this + player_search_fields::ASSIGNED_BASE, 0);
    write_u32(this + player_search_fields::ASSIGNED_OPPONENT, 0);
    write_i32(this + SCOUT_DAY_TARGET_CLUB, -1); // sx17_4: no target club
    1
}

/// Comparer for `player_search_clear_scout_assignment`: it deletes the club record, so only the original runs, and the
/// plan is checked: with a scout list and its scout, every assignment field cleared and no record; otherwise 0.
pub(crate) unsafe extern "thiscall" fn compare_player_search_clear_scout_assignment(this: usize) -> u32 {
    let clears = read_u8(this + player_search_fields::LIST_TYPE) == 4 && get_scout(this) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    let cleared = read_i32(this + player_search_fields::ASSIGNED_CLUB) == -1 && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) == 0
        && read_i32(this + player_search_fields::ASSIGNED_NATION) == -1 && read_i8(this + player_search_fields::ASSIGNED_REGION) == -1
        && read_i32(this + player_search_fields::ASSIGNED_COMP) == -1 && read_u32(this + player_search_fields::ASSIGNED_BASE) == 0
        && read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 && read_i32(this + SCOUT_DAY_TARGET_CLUB) == -1;
    SCOREBOARD.record(if clears { r == 1 && cleared } else { r == 0 }, || format!("player_search_clear_scout_assignment({:#x}) returned {} cleared {}", this, r, cleared));
    r
}

/// The match `k` of a watched player's 10-match ring (`scouted_player_slot`): the oldest is at the ring start.
unsafe fn player_search_slot_match(slot: usize, k: u8) -> usize {
    slot + 6 + k as i8 as usize * 0x29
}

/// Adds a byte field of every stored match of a watched player's ring (signed, as the original's `movsx`).
unsafe fn player_search_slot_sum(slot: usize, field: usize) -> u16 {
    let mut k = read_u8(slot + scouted_player_slot::HISTORY_START);
    let mut sum = 0i32;
    for _ in 0..read_i8(slot + scouted_player_slot::HISTORY_COUNT).max(0) {
        sum += read_i8(player_search_slot_match(slot, k) + field) as i32;
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    sum as u16
}

/// The assists of all the stored matches of a watched player's ring.
/// Original: `player_search_slot_sum_assists` (0x7bd8c0, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_slot_sum_assists(this: usize) -> u16 {
    player_search_slot_sum(this, scouted_match_entry::ASSISTS)
}

/// Comparer for `player_search_slot_sum_assists`: both run, the sums must match.
pub(crate) unsafe extern "thiscall" fn compare_player_search_slot_sum_assists(this: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(%%I2%%)(this);
    let n = player_search_slot_sum_assists(this);
    SCOREBOARD.record(r == n, || format!("player_search_slot_sum_assists({:#x}) original={} rust={}", this, r, n));
    r
}

/// The goals conceded in all the stored matches of a watched player's ring.
/// Original: `player_search_slot_sum_goals_conceded` (0x7bd910, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_slot_sum_goals_conceded(this: usize) -> u16 {
    player_search_slot_sum(this, scouted_match_entry::GOALS_CONCEDED)
}

/// Comparer for `player_search_slot_sum_goals_conceded`: both run, the sums must match.
pub(crate) unsafe extern "thiscall" fn compare_player_search_slot_sum_goals_conceded(this: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(%%I3%%)(this);
    let n = player_search_slot_sum_goals_conceded(this);
    SCOREBOARD.record(r == n, || format!("player_search_slot_sum_goals_conceded({:#x}) original={} rust={}", this, r, n));
    r
}

/// The average match grade (`scouted_match_entry::MATCH_GRADE`, unsigned) of a watched player's stored matches: the
/// sum as a 16-bit signed number over the count, in the x87; `rdata::F64_0` without matches.
/// Original: `player_search_slot_average_rating` (0x7bd960, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_slot_average_rating(this: usize) -> f64 {
    let count = read_i8(this + scouted_player_slot::HISTORY_COUNT);
    let mut k = read_u8(this + scouted_player_slot::HISTORY_START);
    let mut sum = 0i32;
    for _ in 0..count.max(0) {
        sum += read_u8(player_search_slot_match(this, k) + scouted_match_entry::MATCH_GRADE) as i32;
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    if count == 0 {
        return read_f64(rdata::F64_0);
    }
    sum as i16 as f64 / count as f64 // sx17_4: the average grade
}

/// Comparer for `player_search_slot_average_rating`: both run, the averages must be the same bits.
pub(crate) unsafe extern "thiscall" fn compare_player_search_slot_average_rating(this: usize) -> f64 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> f64>(%%I4%%)(this);
    let n = player_search_slot_average_rating(this);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("player_search_slot_average_rating({:#x}) original={} rust={}", this, r, n));
    r
}
