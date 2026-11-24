// --- sx20_1: the daily simulated matches of the clubs outside the playable leagues (simulated_stats.cpp,
// SimulatedStats::play_daily_matches 0x84be20) ---

/// A SimulatedStats group (`SIMULATED_GROUP_SIZE` bytes): +0 the club count, +4 up to `SIMULATED_GROUP_CLUBS` club ids.
/// The SimulatedStats object holds the group count at +0 and the group table at +4.
pub(crate) const SIMULATED_GROUP_SIZE: usize = 0x44;
pub(crate) const SIMULATED_GROUP_CLUBS: usize = 16;
/// The static match record the simulated matches are summarised into (built on the first call, with its two team
/// blocks from `match_record::HOME_TEAM`), and its guard byte (bit 0: built).
const SIMULATED_MATCH_RECORD: usize = 0xdd7168;
const SIMULATED_MATCH_RECORD_GUARD: usize = 0xdd7160;
/// The routines that take a played simulated match (`this`, the fixture, the match record), in the original's order:
/// PlayerStats::record_match, CONTRACT_MANAGER::update_after_match, Injury::apply_match_effects,
/// AWARD_MANAGER::process_fixture and the appearance clauses of TRANSFER_MANAGER.
const SIMULATED_MATCH_RECORDERS: [(usize, usize); 5] = [
    (G_PLAYER_STATS, 0x7c0d70),
    (G_CONTRACT_MANAGER, 0x4ca6d0),
    (G_INJURY, 0x62c080),
    (G_AWARD_MANAGER, 0x4134c0),
    (G_TRANSFER_MANAGER, 0x8c39f0),
];

/// The address of group `g` of a SimulatedStats object.
pub(crate) unsafe fn simulated_group(this: usize, g: usize) -> usize {
    read_usize(this + 4) + g * SIMULATED_GROUP_SIZE
}

/// The address of club slot `k` of group `g`.
unsafe fn simulated_group_slot(this: usize, g: usize, k: usize) -> usize {
    simulated_group(this, g) + 4 + k * 4
}

/// A random index near `index` among `count`: `index` minus random(count / 150) plus random(count / 150), drawn again
/// until it falls from 0 to `count` less one (the same draw the constructor's shuffle uses).
pub(crate) unsafe fn simulated_nearby_index(index: i32, count: impl Fn() -> i32) -> i32 {
    loop {
        let back = exe::utils_random(count() / 150);
        let near = index.wrapping_sub(back).wrapping_add(exe::utils_random(count() / 150));
        if near >= 0 && near < count() {
            return near;
        }
    }
}

/// Mixes the groups: as many times as there are groups, a random group and one near it (`simulated_nearby_index`)
/// swap a random club each.
unsafe fn simulated_stats_mix_groups(this: usize) {
    let groups = || read_i32(this);
    let mut i = 0;
    while i < groups() {
        let a = exe::utils_random(groups());
        let b = simulated_nearby_index(a, groups);
        let ka = exe::utils_random(read_i32(simulated_group(this, a as usize)));
        let kb = exe::utils_random(read_i32(simulated_group(this, b as usize)));
        let (sa, sb) = (simulated_group_slot(this, a as usize, ka as usize), simulated_group_slot(this, b as usize, kb as usize));
        let club = read_u32(sa);
        write_u32(sa, read_u32(sb));
        write_u32(sb, club);
        i += 1;
    }
}

/// How many players of `club_id` could play a simulated match today: -1 when the club has a fixture from two days
/// ago to three days ahead (`exe::fixture_manager_find_fixtures` with competition -2) or no nation; otherwise each of
/// the 50 squad slots with a player sheet that is not out injured (injury status 2) and not away on international
/// duty today.
unsafe fn simulated_stats_free_players(club_id: i32) -> i8 {
    let mut before = [0u8; 8];
    let mut after = [0u8; 8];
    let from = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 2);
    let to = exe::date_add_days(G_CURRENT_DATE, after.as_mut_ptr() as usize, 3); // sx20_1: three days ahead
    let busy = exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(from), read_u32(from + 2), read_u32(to),
        read_u32(to + 2), club_id as u32, 0usize, u32::MAX, 0xffff_fffe, u32::MAX);
    let club = club_at(club_id as isize);
    if busy != 0 || club.nation().is_null() {
        return -1;
    }
    let mut free = 0i8;
    for k in 0..50 {
        let person = Staff(club.squad(k));
        if person.is_null() || person.player().is_null() {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 { // sx20_1: out injured
            continue;
        }
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0) == 0 {
            free += 1;
        }
    }
    free
}

/// Plays one simulated match between the clubs `home` and `away` in the fixture `game` (kept by the caller, as the
/// original's stack slot): competition -2, no stadium, category 2, both side marks 1, score 0-0 and no half-time,
/// extra time, shoot-out or aggregate. Returns false when the match engine could not be built (the error box) or
/// set up; the original then retries the same pair.
unsafe fn simulated_stats_play_match(game: usize, home: i32, away: i32) -> bool {
    write_i32(game + fixture::HOME_CLUB_ID, home);
    write_u8(game + fixture::SQUAD_RULES, 0);
    write_i32(game + fixture::AWAY_CLUB_ID, away);
    write_usize(game + fixture::HOME_CLUB, club_at(home as isize).addr());
    write_usize(game + fixture::AWAY_CLUB, club_at(away as isize).addr());
    write_i32(game + fixture::COMP_ID, -2);
    write_usize(game + fixture::COMPETITION, 0);
    write_u8(game + fixture::HOME_SIDE_MARK, 1);
    write_u8(game + fixture::AWAY_SIDE_MARK, 1);
    write_usize(game + fixture::STADIUM, 0);
    write_i32(game + fixture::STADIUM_ID, -1);
    write_u8(game + fixture::NEUTRAL_VENUE, 0);
    write_u8(game + fixture::CATEGORY, 2); // sx20_1: the category of a simulated match
    write_u8(game + fixture::HOME_GOALS, 0);
    write_u8(game + fixture::AWAY_GOALS, 0);
    for o in fixture::HOME_GOALS_HALF_TIME..=fixture::AWAY_AGGREGATE {
        write_u8(game + o, 0xff);
    }
    exe::virtual_staff_fill_fixture_squads(read_usize(G_VIRTUAL_STAFF), game, -2, 0);
    let mut engine = exe::crt_operator_new(0xf63b);
    if engine != 0 {
        engine = exe::new_cm3_match_engine(engine);
    }
    if engine == 0 {
        show_error_at(0xa8278c, 0x15e);
        return false;
    }
    if exe::cm3_match_engine_init_match(engine, game, 1, 0usize, 0) == 0 {
        return false;
    }
    covered!("sx20_1 simulated match");
    exe::cm3_match_engine_run_match_steps(engine, -1);
    exe::match_stats_fill_match_record(read_usize(G_MATCH_STATS), engine, SIMULATED_MATCH_RECORD, 0);
    for (owner, routine) in SIMULATED_MATCH_RECORDERS { // sx20_1: the routines that take the match
        let record: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(routine);
        record(read_usize(owner), game, SIMULATED_MATCH_RECORD);
    }
    exe::drop_cm3_match_engine(engine);
    exe::crt_operator_delete(engine);
    exe::virtual_staff_release_fixture(read_usize(G_VIRTUAL_STAFF), game);
    true
}

/// Plays one group's matches today: each club's free players (`simulated_stats_free_players`), the clubs sorted by
/// them (most first), the clubs with free players shuffled among themselves (count² random swaps, when more than two),
/// then the clubs paired in order (first with second, third with fourth...) while the first of the pair has free
/// players and the second is not busy.
unsafe fn simulated_stats_play_group(this: usize, g: usize, game: usize) {
    let count = || read_i32(simulated_group(this, g));
    let mut clubs = [0i32; SIMULATED_GROUP_CLUBS];
    let mut free = [0i8; SIMULATED_GROUP_CLUBS];
    let mut with_free = 0i8;
    let mut k = 0;
    while k < count() as usize {
        clubs[k] = read_i32(simulated_group_slot(this, g, k));
        free[k] = simulated_stats_free_players(clubs[k]);
        if free[k] > 0 {
            with_free += 1;
        }
        k += 1;
    }
    let n = count() as usize;
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if free[i] < free[j] { // sx20_1: most free players first
                free.swap(i, j);
                clubs.swap(i, j);
            }
        }
    }
    let rounds = with_free as i32 * with_free as i32;
    if rounds > 0 && with_free > 2 {
        for _ in 0..rounds {
            let a = exe::utils_random(with_free as i32) as usize;
            let b = exe::utils_random(with_free as i32) as usize;
            free.swap(a, b);
            clubs.swap(a, b);
        }
    }
    let mut k = 0usize;
    while (k as i32) < count() - 1 {
        if free[k] <= 0 || free[k + 1] <= -1 { // sx20_1: a pair needs free players
            break;
        }
        if simulated_stats_play_match(game, clubs[k], clubs[k + 1]) {
            k += 2;
        }
    }
}

/// Plays the day's simulated matches of the clubs outside the playable leagues. On the first call it builds the
/// static match record (its two team blocks, `match_team_block_construct`/`match_team_block_destruct`) and registers
/// its destruction (0x84c500) with `atexit`. Then it mixes the groups (`simulated_stats_mix_groups`) and plays the
/// groups whose index matches today's day of the year modulo 7 (`simulated_stats_play_group`), except in months 5
/// and 6 (`Date::get_month`). Every match is a fresh fixture (competition -2, category 2) played to the end by a new
/// match engine, then summarised into the static record and passed to the player statistics, contracts, injuries,
/// awards and appearance clauses (`SIMULATED_MATCH_RECORDERS`).
/// Defects of the original: when the match engine cannot be built (the error box, line 0x15e) or set up
/// (`init_match` 0, leaking the engine), the same pair is tried again without end; a club with 0 free players
/// can still be the second of a pair (only -1, a busy club, stops the pairing); the fixture's match statistics slot,
/// dates, referee, round and rule fields, stage, result marks, time slice, stage index and flags byte are never set:
/// the original passes what its stack holds there (the copy, what the previous match of the call left, else 0).
/// Original: `SimulatedStats::play_daily_matches` (0x84be20, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_play_daily_matches(this: usize) {
    let guard = read_u8(SIMULATED_MATCH_RECORD_GUARD);
    if guard & 1 == 0 {
        write_u8(SIMULATED_MATCH_RECORD_GUARD, guard | 1);
        exe::crt_vector_constructor_iterator(SIMULATED_MATCH_RECORD + match_record::HOME_TEAM, match_record::SIDE_SIZE as u32, 2, 0x453c30usize, 0x453c10usize);
        exe::crt_atexit(0x84c500usize);
    }
    if read_i32(this) > 0 {
        simulated_stats_mix_groups(this);
    }
    let mut game = [0u8; fixture::SIZE + 1];
    let mut g = 0i32;
    while g < read_i32(this) {
        let today = read_i16(G_CURRENT_DATE) as i32;
        if today % 7 == g % 7 { // sx20_1: each group plays one day in seven
            if exe::date_get_month(G_CURRENT_DATE) as i8 <= 4 || exe::date_get_month(G_CURRENT_DATE) as i8 >= 7 {
                simulated_stats_play_group(this, g as usize, game.as_mut_ptr() as usize);
            }
        }
        g += 1;
    }
}

/// Comparer for `simulated_stats_play_daily_matches`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_play_daily_matches(this: usize) {
    // simplification: plays matches and draws random numbers (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
