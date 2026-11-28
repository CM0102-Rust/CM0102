use super::*;

/// Writes the count (+0) and the 0x44-byte records from `[+4]` into the simulated_stats.dat item; on error the "Error"
/// box without closing the item.
///
/// `SimulatedStats::save(save)`: writes into the `simulated_stats.dat` item the count (+0, u32) and the 0x44-byte
/// records of `[+4]`. Error on open or write: the "Error" box (simulated_stats.cpp 0x209, 0x210, 0x218), without
/// closing the item. Does not check for a null container.
/// Original: `SimulatedStats::save` (0x84c680, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_save(this: usize, save: usize) {
    const FONTE: usize = 0xa8278c;
    covered!("cpx_7 save simulated stats");
    let mut z = zip_open_item(save, 0xa827c0);
    if z == 0 {
        show_error_at(FONTE, 0x209);
        return;
    }
    if zip_write(this, 4, 1, z) != 1 {
        show_error_at(FONTE, 0x210);
        return;
    }
    if zip_write(read_usize(this + 4), 0x44, read_usize(this) as u32, z) != read_usize(this) as u32 { // cpx7: os registros simulados
        show_error_at(FONTE, 0x218);
        return;
    }
    zip_close(&mut z);
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SIMULATED_STATS_PLAY_DAILY_MATCHES)(this)
}

/// Builds the simulated statistics: first waits for the game CD (unless the interactive mode flag
/// `G_INTERACTIVE_MODE_FLAG` is set: `wait_for_game_cd`). With a saved game `save`, loads the groups from it
/// (`SimulatedStats::load`). Otherwise takes the clubs outside the played competitions that are not national teams
/// (`simulated_stats_collect_clubs`; without memory for the list, the error box, line 0x49, the fatal cleanup and
/// `exit(-1)`), the group count = ⌈count / 16⌉ at +0, sorts them by reputation, shuffles them a little and deals them
/// into a new table of groups at +4 (`simulated_stats_fill_groups`). Returns `this`.
/// Defects of the original: with fewer than 16 × (16 − count % 16) clubs the short groups ask for more clubs than
/// there are (the box, then ids read past the list); the group table is not checked for null.
/// Original: `SimulatedStats::SimulatedStats` (0x84b940, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        wait_for_game_cd(SIMULATED_STATS_CD_DRIVE_TYPE_JUMP, SIMULATED_STATS_CD_FOUND_TEST);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SIMULATEDSTATS_LOAD); // SimulatedStats::load
        load(this, save);
        return this;
    }
    let list = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    if list == 0 {
        fatal_error_at(SIMULATED_STATS_SOURCE, 0x49);
    }
    let n = simulated_stats_collect_clubs(list);
    let rest = n % 16;
    write_i32(this, n / 16 + (rest != 0) as i32);
    covered!("sx20_2 simulated stats groups");
    simulated_stats_sort_by_reputation(list, n);
    simulated_stats_shuffle_clubs(list, n);
    write_usize(this + 4, exe::crt_malloc((read_i32(this) as u32).wrapping_mul(SIMULATED_GROUP_SIZE as u32)));
    simulated_stats_fill_groups(this, list, n, rest);
    exe::crt_free(list);
    this
}

/// Frees the group table (+4) when there is one.
/// Original: `SimulatedStats::~SimulatedStats` (0x84be10, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_destructor(this: usize) {
    if read_usize(this + 4) != 0 {
        exe::crt_free(read_usize(this + 4));
    }
}

/// Comparer for `simulated_stats_simulated_stats`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    // simplification: allocates and draws random numbers (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_SIMULATED_STATS_SIMULATED_STATS)(this, save)
}

/// Comparer for `simulated_stats_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_destructor(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SIMULATED_STATS_DESTRUCTOR)(this)
}

/// Mixes the groups: as many times as there are groups, a random group and one near it (`simulated_nearby_index`)
/// swap a random club each.
pub(crate) unsafe fn simulated_stats_mix_groups(this: usize) {
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

/// Plays one group's matches today: each club's free players (`simulated_stats_free_players`), the clubs sorted by
/// them (most first), the clubs with free players shuffled among themselves (count² random swaps, when more than two),
/// then the clubs paired in order (first with second, third with fourth...) while the first of the pair has free
/// players and the second is not busy.
pub(crate) unsafe fn simulated_stats_play_group(this: usize, g: usize, game: usize) {
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

/// The CD search some constructors start with: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box (OK) in between. A
/// no-CD exe patched in the calling routine is honoured by reading its two patched bytes: the drive type's `jne` at
/// `drive_type_jump` made `nop` (any drive) and the found test at `found_test` no longer a `test` (always found). The
/// same search as `club_history_find_cd`, for any routine's addresses.
pub(crate) unsafe fn wait_for_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
    loop {
        // simplification: the drive list starts zeroed (as club_history_find_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                if any_drive || kind(root) == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx20_2 insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// Writes into `list` the ids of the clubs that play simulated matches: neither their division nor their reserve
/// division is played and they are not a national or all-star team. Returns how many.
pub(crate) unsafe fn simulated_stats_collect_clubs(list: usize) -> i32 {
    let mut n = 0i32;
    for i in 0..read_i32(G_CLUB_COUNT) {
        let club = club_at(i as isize);
        if simulated_stats_is_played(club.division()) || simulated_stats_is_played(club.reserve_division()) {
            continue;
        }
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
            continue;
        }
        write_i32(list + 4 * n as usize, read_i32(club.addr())); // the club id (+0)
        n += 1;
    }
    n
}

/// Sorts the club ids of `list` by reputation, highest first (a pairwise exchange sort, as the original).
pub(crate) unsafe fn simulated_stats_sort_by_reputation(list: usize, n: i32) {
    for i in 0..(n - 1).max(0) as usize {
        for j in i + 1..n as usize {
            let (a, b) = (read_i32(list + 4 * i), read_i32(list + 4 * j));
            if club_at(a as isize).reputation() < club_at(b as isize).reputation() { // sx20_2: highest reputation first
                write_i32(list + 4 * i, b);
                write_i32(list + 4 * j, a);
            }
        }
    }
}

/// Shuffles the sorted clubs a little: n / 2 times a random club swaps with one near it (`simulated_nearby_index`).
pub(crate) unsafe fn simulated_stats_shuffle_clubs(list: usize, n: i32) {
    let swaps = n / 2; // sx20_2: how many swaps
    for _ in 0..swaps {
        let a = exe::utils_random(n);
        let b = simulated_nearby_index(a, || n);
        let club = read_i32(list + 4 * b as usize);
        write_i32(list + 4 * b as usize, read_i32(list + 4 * a as usize));
        write_i32(list + 4 * a as usize, club);
    }
}

/// Deals the clubs of `list` in order into the groups: 16 each, except that, when the club count is not a multiple
/// of 16 (`rest` = count % 16), the first 16 − `rest` groups take 15. Running out of clubs shows the error box (line
/// 0x95) once and goes on reading past the list.
pub(crate) unsafe fn simulated_stats_fill_groups(this: usize, list: usize, n: i32, rest: i32) {
    let mut next = 0i32;
    for g in 0..read_i32(this) {
        let group = simulated_group(this, g as usize);
        write_i32(group, 0);
        for slot in 0..SIMULATED_GROUP_CLUBS as i32 {
            if next == n {
                show_error_at(SIMULATED_STATS_SOURCE, 0x95);
            }
            let count = read_i32(group);
            write_i32(group + 4 + 4 * count as usize, read_i32(list.wrapping_add((next as usize).wrapping_mul(4))));
            write_i32(group, count + 1);
            next += 1;
            if rest != 0 && g < 16 - rest && slot == 14 { // sx20_2: the short groups take 15
                break;
            }
        }
    }
}

/// The address of group `g` of a SimulatedStats object.
pub(crate) unsafe fn simulated_group(this: usize, g: usize) -> usize {
    read_usize(this + 4) + g * SIMULATED_GROUP_SIZE
}

/// The address of club slot `k` of group `g`.
pub(crate) unsafe fn simulated_group_slot(this: usize, g: usize, k: usize) -> usize {
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

/// How many players of `club_id` could play a simulated match today: -1 when the club has a fixture from two days
/// ago to three days ahead (`exe::fixture_manager_find_fixtures` with competition -2) or no nation; otherwise each of
/// the 50 squad slots with a player sheet that is not out injured (injury status 2) and not away on international
/// duty today.
pub(crate) unsafe fn simulated_stats_free_players(club_id: i32) -> i8 {
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
pub(crate) unsafe fn simulated_stats_play_match(game: usize, home: i32, away: i32) -> bool {
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

/// Whether a club's division or reserve division is a competition the game plays (its object in
/// `G_COMP_OBJECT_TABLE`).
pub(crate) unsafe fn simulated_stats_is_played(division: Competition) -> bool {
    !division.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(division.addr()) as usize).wrapping_mul(4))) != 0
}

/// A SimulatedStats group (`SIMULATED_GROUP_SIZE` bytes): +0 the club count, +4 up to `SIMULATED_GROUP_CLUBS` club ids.
/// The SimulatedStats object holds the group count at +0 and the group table at +4.
pub(crate) const SIMULATED_GROUP_SIZE: usize = 0x44;

/// The static match record the simulated matches are summarised into (built on the first call, with its two team
/// blocks from `match_record::HOME_TEAM`), and its guard byte (bit 0: built).
pub(crate) const SIMULATED_MATCH_RECORD: usize = 0xdd7168;

pub(crate) const SIMULATED_MATCH_RECORD_GUARD: usize = 0xdd7160;

/// The routines that take a played simulated match (`this`, the fixture, the match record), in the original's order:
/// PlayerStats::record_match, CONTRACT_MANAGER::update_after_match, Injury::apply_match_effects,
/// AWARD_MANAGER::process_fixture and the appearance clauses of TRANSFER_MANAGER.
pub(crate) const SIMULATED_MATCH_RECORDERS: [(usize, usize); 5] = [
    (G_PLAYER_STATS, 0x7c0d70),
    (G_CONTRACT_MANAGER, 0x4ca6d0),
    (G_INJURY, 0x62c080),
    (G_AWARD_MANAGER, 0x4134c0),
    (G_TRANSFER_MANAGER, 0x8c39f0),
];

/// simulated_stats.cpp, the source path of the error boxes.
pub(crate) const SIMULATED_STATS_SOURCE: usize = 0xa8278c;

/// The two bytes of the constructor's CD search that a no-CD exe patches (as `cm0102-1080`: the drive type's `jne`
/// made `nop`, the found test made `xor`).
pub(crate) const SIMULATED_STATS_CD_DRIVE_TYPE_JUMP: usize = 0x84b9a0;

pub(crate) const SIMULATED_STATS_CD_FOUND_TEST: usize = 0x84b9e3;

pub(crate) const SIMULATED_GROUP_CLUBS: usize = 16;
