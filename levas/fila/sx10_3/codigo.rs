// ---------- sx10_3: the fog of war maps: what each human manager knows of each person (fog_of_war.cpp) ----------

/// FOG_OF_WAR_MAP::FOG_OF_WAR_MAP (thiscall(map, manager) ret 4, returns the map).
pub(crate) const FN_FOG_OF_WAR_MAP_NEW: usize = 0x5aff00;
/// fog_of_war_initial_knowledge_level (thiscall(map, manager, person) ret 8, the level in al).
pub(crate) const FN_FOG_OF_WAR_INITIAL_KNOWLEDGE_LEVEL: usize = 0x5b03e0;
/// fog_of_war_minimum_knowledge_level (thiscall(map, manager, person) ret 8, the level in al).
pub(crate) const FN_FOG_OF_WAR_MINIMUM_KNOWLEDGE_LEVEL: usize = 0x5b06e0;
/// player_stats_years_since_at_club (stdcall(person, club), the years as a word; -1 never there).
pub(crate) const FN_PLAYER_STATS_YEARS_SINCE_AT_CLUB: usize = 0x7c6e30;
/// "Initialising game data", the progress text while a map is built.
const FOG_INITIALISING_TEXT: usize = 0x9cc2ac;
/// A knowledge level drops by one when it was last raised more than this many days ago.
const FOG_FORGET_AFTER_DAYS: i32 = 0x21c;
/// The knowledge levels: 0 unknown, 1 and 2 partly known, 3 fully known.
const FOG_FULLY_KNOWN: u8 = 3;
/// Above this reputation (`current_reputation`, player or non-player) a person is known nationally.
const FOG_KNOWN_REPUTATION: i16 = 0x109a;
/// Above this world reputation a person is known everywhere.
const FOG_FAMOUS_WORLD_REPUTATION: i16 = 0x1482;
/// Above this score (the nation's reputation / 10 × the person's caps) an international is known everywhere.
const FOG_FAMOUS_CAPS_SCORE: f64 = 8.5;
/// A national team's nation with fewer people than this in the database is small enough for its manager to know all
/// of its people.
const FOG_SMALL_NATION_STAFF: i32 = 1000;

/// The club of a person's contract, as the original works it out inline: `CONTRACT_MANAGER::get_contract` (its result
/// left in `G_CONTRACT_LOOKUP_RESULT`) and the club of the contract's club id; no contract: 0.
unsafe fn fog_contract_club(person: Staff) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if c.is_null() {
        return 0;
    }
    club_at(c.club() as i32 as isize).addr()
}

/// The B team (or first team) linked to `club` (`club_get_linked_b_team(club, &out, 1)`).
unsafe fn fog_linked_club(club: usize) -> usize {
    let mut out: u32 = 0;
    exe::club_get_linked_b_team(club, &mut out, 1)
}

/// How many years since `person` was at `club` (`player_stats_years_since_at_club`): 0 there now, -1 never.
unsafe fn fog_years_since_at_club(person: Staff, club: Club) -> i16 {
    let years: unsafe extern "stdcall" fn(usize, usize) -> i16 = core::mem::transmute(FN_PLAYER_STATS_YEARS_SINCE_AT_CLUB);
    years(person.addr(), club.addr())
}

/// Whether `person` was at `club` less than `years` years ago (never there: no).
unsafe fn fog_was_at_club_within(person: Staff, club: Club, years: i16) -> bool {
    if club.is_null() || person.is_null() {
        return false;
    }
    let y = fog_years_since_at_club(person, club);
    y != -1 && y < years
}

/// The person's current reputation, of the player sheet or else of the non-player sheet, is above `limit`.
unsafe fn fog_reputation_above(person: Staff, limit: i16) -> bool {
    if !person.player().is_null() {
        return person.player().current_reputation() > limit;
    }
    let sheet = person.non_player();
    !sheet.is_null() && sheet.current_reputation() > limit
}

/// The person is known everywhere: an international of a big nation (the nation's reputation / 10, truncated, times
/// the caps, above 8.5) or a world reputation above 0x1482 (the player sheet's, or else the non-player sheet's). With
/// no person or no nation both checks are skipped.
unsafe fn fog_is_famous(person: Staff) -> bool {
    if person.is_null() || person.nation().is_null() {
        return false;
    }
    let score = (person.nation().reputation() as i32 / 10) as f64 * person.int_apps() as f64;
    if score > FOG_FAMOUS_CAPS_SCORE {
        return true;
    }
    if !person.player().is_null() {
        return person.player().world_reputation() > FOG_FAMOUS_WORLD_REPUTATION;
    }
    let sheet = person.non_player();
    !sheet.is_null() && sheet.world_reputation() > FOG_FAMOUS_WORLD_REPUTATION
}

/// Rule: a new map holds, for every person, what the manager knows of him at the start.
///
/// `FOG_OF_WAR_MAP::FOG_OF_WAR_MAP(manager)`: the knowledge grid (+0) of `G_STAFF_COUNT` entries of 9 bytes (`malloc`).
/// With the grid and a manager: the manager (+4), the progress text "Initialising game data" (`langlib_sprintf`) shown
/// at 0 of the people count; then each person, in table order, gets today's date (`G_CURRENT_DATE`) at +1 of his entry
/// and the level `fog_of_war_initial_knowledge_level(manager, person)` at +0, the progress moving every 1000 people; at
/// the end, the progress at the count. Returns `this`.
///
/// Defect of the original: without a grid or without a manager the manager field (+4) is left as it was (the load path
/// builds maps without a manager and reads the manager afterwards).
/// Original: `FOG_OF_WAR_MAP::FOG_OF_WAR_MAP` (0x5aff00, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_new(this: usize, manager: usize) -> usize {
    let grid = exe::crt_malloc((read_i32(G_STAFF_COUNT) as u32).wrapping_mul(9));
    write_usize(this, grid); // +0: the knowledge grid
    if grid == 0 || manager == 0 {
        return this;
    }
    write_usize(this + 4, manager); // +4: the map's manager
    let mut text = [0u8; 0xc8];
    story_lang!(text.as_mut_ptr(), FOG_INITIALISING_TEXT);
    let progress = |done: i32| {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, done, read_u32(G_STAFF_COUNT), -1)
    };
    progress(0);
    let initial: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_FOG_OF_WAR_INITIAL_KNOWLEDGE_LEVEL);
    let mut person = read_usize(G_STAFF_TABLE);
    let mut k: i32 = 0;
    while k < read_i32(G_STAFF_COUNT) {
        let mut copy = [0u8; 8];
        exe::date_operator_3d(read_usize(this) + 9 * k as usize + 1, copy.as_mut_ptr() as usize, G_CURRENT_DATE);
        let level = initial(this, read_usize(this + 4), person);
        write_u8(read_usize(this) + 9 * k as usize, level); // sx10_3: the starting level
        if k % 1000 == 0 {
            progress(k);
        }
        k += 1;
        person += staff::SIZE;
    }
    let count = read_i32(G_STAFF_COUNT);
    progress(count);
    this
}

/// Comparer for `fog_of_war_map_new`.
///
/// It allocates the grid and moves the progress bar: it cannot run twice. Only the original runs; the copy is proven by
/// a game with attribute masking on (the copy builds each human manager's map).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_new(this: usize, manager: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, manager)
}

/// Rule: a human manager gets his own map, in the slot of his place at the end of the people table.
///
/// `FOG_OF_WAR::add_manager_map(manager)`: no manager: nothing. Otherwise a new map for him (`operator new` of 8 bytes
/// and the map constructor; a failed allocation leaves 0) in slot `G_STAFF_COUNT - id - 1`, over whatever was there.
/// Original: `FOG_OF_WAR::add_manager_map` (0x5afd70, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_add_manager_map(this: usize, manager: usize) {
    if manager == 0 {
        return;
    }
    let slot = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    let map = exe::crt_operator_new(8);
    let map = if map == 0 {
        0
    } else {
        let new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FOG_OF_WAR_MAP_NEW);
        new(map, manager)
    };
    write_usize(this.wrapping_add((slot as usize).wrapping_mul(4)), map);
}

/// Comparer for `fog_of_war_add_manager_map`.
///
/// It allocates the map: it cannot run twice. Only the original runs; the copy is proven by adding a human manager to a
/// game with attribute masking on.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_add_manager_map(this: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, manager)
}

/// Rule: every day a manager forgets a little of whom he has not seen for 540 days, never below what he always knows.
///
/// `FOG_OF_WAR_MAP::daily_update()`: for each person, in table order: a known entry (level ≠ 0) last raised more than
/// 540 days before today (`Date::days_since` on `G_CURRENT_DATE`) gets today's date and its level lowered by one; then
/// the level is raised to `fog_of_war_minimum_knowledge_level(manager, person)` when below it (signed bytes).
/// Original: `FOG_OF_WAR_MAP::daily_update` (0x5b0260, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_daily_update(this: usize) {
    let minimum: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_FOG_OF_WAR_MINIMUM_KNOWLEDGE_LEVEL);
    let entry = |k: i32| read_usize(this) + 9 * k as usize;
    let mut person = read_usize(G_STAFF_TABLE);
    let mut k: i32 = 0;
    while k < read_i32(G_STAFF_COUNT) {
        let e = entry(k);
        if read_u8(e) != 0 && exe::date_days_since(G_CURRENT_DATE, read_u32(e + 1), read_u32(e + 5)) > FOG_FORGET_AFTER_DAYS { // sx10_3: forgotten after 540 days
            covered!("sx10_3 knowledge forgotten");
            let mut copy = [0u8; 8];
            exe::date_operator_3d(entry(k) + 1, copy.as_mut_ptr() as usize, G_CURRENT_DATE);
            write_u8(entry(k), read_u8(entry(k)).wrapping_sub(1));
        }
        let floor = minimum(this, read_usize(this + 4), person) as i8;
        if (read_u8(entry(k)) as i8) < floor {
            write_u8(entry(k), floor as u8);
        }
        k += 1;
        person += staff::SIZE;
    }
}

/// Comparer for `fog_of_war_map_daily_update`: side by side over the knowledge grid and the contract lookup global.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let regions = [(read_usize(this), 9 * read_i32(G_STAFF_COUNT).max(0) as usize), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || fog_of_war_map_daily_update(this));
    SCOREBOARD.record(same, || format!("fog_of_war_map_daily_update({:#x}): {:?}", this, first_differences(&mo, &mr)));
}

/// Rule: how well a human manager knows a person right now.
///
/// `FOG_OF_WAR_MAP::get_knowledge_level(person)`: no person or a map without a manager (+4): 0. A person beyond the
/// table (`G_STAFF_COUNT`) or attribute masking off: 3 (fully known). Also 3 when the manager has a club and the person
/// is at it, or the person's contract club (`get_contract`, left in `G_CONTRACT_LOOKUP_RESULT`) is the manager's club;
/// when both have the same national team job; when both have clubs and the person's linked club
/// (`club_get_linked_b_team`) is the manager's club. Otherwise the level stored for the person (+0 of his 9-byte entry).
/// Original: `FOG_OF_WAR_MAP::get_knowledge_level` (0x5b02f0, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_get_knowledge_level(this: usize, person: usize) -> u8 {
    let person = Staff(person);
    if person.is_null() || read_usize(this + 4) == 0 {
        return 0;
    }
    let manager = Staff(read_usize(this + 4)); // +4: the map's manager
    if read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) || read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 {
        return FOG_FULLY_KNOWN;
    }
    if !manager.club_job().is_null() {
        if person.club_job() == manager.club_job() || fog_contract_club(person) == manager.club_job().addr() {
            covered!("sx10_3 known at the manager's club");
            return FOG_FULLY_KNOWN;
        }
    }
    if !person.national_job().is_null() && person.national_job() == manager.national_job() { // sx10_3: same national team
        return FOG_FULLY_KNOWN;
    }
    if !person.club_job().is_null() && !manager.club_job().is_null() && fog_linked_club(person.club_job().addr()) == manager.club_job().addr() {
        return FOG_FULLY_KNOWN;
    }
    read_u8(read_usize(this).wrapping_add((read_i32(person.addr()) as usize).wrapping_mul(9)))
}

compare_value!(compare_fog_of_war_map_get_knowledge_level, fog_of_war_map_get_knowledge_level, %%I3%%, (person: usize) -> u8);

/// The level from reputation, age and the manager's club, the same block for several of the links of
/// `fog_of_war_initial_knowledge_level`: a reputation above 0x109a: 3; older than `age_limit`: 2; at the manager's club
/// less than 3 years ago: 2; otherwise 1.
unsafe fn fog_initial_by_standing(person: Staff, manager: Staff, age_limit: i8) -> u8 {
    if fog_reputation_above(person, FOG_KNOWN_REPUTATION) {
        return FOG_FULLY_KNOWN;
    }
    if (person.age() as i8) > age_limit {
        return 2;
    }
    if fog_was_at_club_within(person, manager.club_job(), 3) {
        return 2;
    }
    1
}

/// Rule: what a manager knows of a person when his map is made: everything of non-players, his national team, famous
/// people, older ones of his division and players who were at his club; partly those linked to his nation; nothing of
/// the rest.
///
/// `fog_of_war_initial_knowledge_level(manager, person)`, in order:
/// - a person without a player sheet: 3; the same national team job as the manager: 3;
/// - the links: the person's nation is the manager's (same nation); the manager's club and the person's club are in the
///   same nation, or the manager's national team club is in the person's club's nation (national link); the person's
///   club is in the manager's nation; the manager's club is in the person's nation; both clubs in the same division;
/// - famous (`fog_is_famous`; for this routine the player sheet's world reputation): 3; same division and older than
///   19: 3; at the manager's club less than 2 years ago (`player_stats_years_since_at_club`): 3;
/// - national link: `fog_initial_by_standing` with age 21; the person's club in the manager's nation: with age 23;
/// - neither the same nation nor the manager's club in the person's nation: 1 when the manager's national team is of a
///   nation with fewer than 1000 people and that nation is the person's, otherwise 0;
/// - otherwise `fog_initial_by_standing` with age 23.
/// Original: `fog_of_war_initial_knowledge_level` (0x5b03e0, fog_of_war.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fog_of_war_initial_knowledge_level(_this: usize, manager: usize, person: usize) -> u8 {
    let (manager, person) = (Staff(manager), Staff(person));
    if person.player().is_null() {
        return FOG_FULLY_KNOWN;
    }
    let same_nation = person.nation() == manager.nation();
    if !person.national_job().is_null() && person.national_job() == manager.national_job() {
        return FOG_FULLY_KNOWN;
    }
    let (manager_club, person_club) = (manager.club_job(), person.club_job());
    let national_link = (!manager_club.is_null() && !person_club.is_null() && manager_club.nation() == person_club.nation())
        || (!manager.national_job().is_null() && !person_club.is_null() && manager.national_job().nation() == person_club.nation());
    let club_in_manager_nation = !person_club.is_null() && person_club.nation() == manager.nation();
    let manager_club_in_person_nation = !manager_club.is_null() && manager_club.nation() == person.nation();
    let same_division = !manager_club.is_null() && !person_club.is_null() && manager_club.division() == person_club.division();
    if fog_is_famous(person) {
        covered!("sx10_3 famous person");
        return FOG_FULLY_KNOWN;
    }
    if same_division && (person.age() as i8) > 0x13 { // sx10_3: older players of the division
        return FOG_FULLY_KNOWN;
    }
    if fog_was_at_club_within(person, manager_club, 2) {
        return FOG_FULLY_KNOWN;
    }
    if national_link {
        return fog_initial_by_standing(person, manager, 0x15);
    }
    if club_in_manager_nation {
        return fog_initial_by_standing(person, manager, 0x17);
    }
    if !same_nation && !manager_club_in_person_nation {
        let team = manager.national_job();
        if !team.is_null() {
            let nation = team.nation();
            if !nation.is_null() && nation.number_staff() < FOG_SMALL_NATION_STAFF && nation == person.nation() {
                return 1;
            }
        }
        return 0;
    }
    fog_initial_by_standing(person, manager, 0x17)
}

compare_value!(compare_fog_of_war_initial_knowledge_level, fog_of_war_initial_knowledge_level, %%I4%%, (manager: usize, person: usize) -> u8);

/// The floor from reputation and age, the same block for two of the links of `fog_of_war_minimum_knowledge_level`: a
/// reputation above 0x109a: 2; older than 21: 1; otherwise none (the next link decides).
unsafe fn fog_minimum_by_standing(person: Staff) -> Option<u8> {
    if fog_reputation_above(person, FOG_KNOWN_REPUTATION) {
        return Some(2);
    }
    if (person.age() as i8) > 0x15 {
        return Some(1);
    }
    None
}

/// Rule: the least a manager keeps knowing of a person, however long since he saw him: everything of famous people, his
/// own players (and those of the linked clubs) and his national team; less of his division, his nation and the clubs
/// around.
///
/// `fog_of_war_minimum_knowledge_level(manager, person)`, in order:
/// - the links: the same nation; both clubs in the same nation; the manager's club in the person's nation; the person's
///   club in the manager's nation; both clubs in the same division;
/// - famous (`fog_is_famous`): 3; the person at the manager's club: 3; the same national team job: 3; the person's
///   contract club (`get_contract`, left in `G_CONTRACT_LOOKUP_RESULT`) is the manager's club: 3 (also when both are
///   none); the club linked to the person's club, or to his contract club (`club_get_linked_b_team`), is the manager's
///   club: 3;
/// - same division: 2;
/// - clubs in the same nation: `fog_minimum_by_standing`; the same nation or the manager's club in the person's nation:
///   the same;
/// - the person's club in the manager's nation: 1 with a reputation above 0x109a, otherwise 0;
/// - otherwise 0.
/// Original: `fog_of_war_minimum_knowledge_level` (0x5b06e0, fog_of_war.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fog_of_war_minimum_knowledge_level(_this: usize, manager: usize, person: usize) -> u8 {
    let (manager, person) = (Staff(manager), Staff(person));
    let (manager_club, person_club) = (manager.club_job(), person.club_job());
    let same_nation = manager.nation() == person.nation();
    let clubs_same_nation = !manager_club.is_null() && !person_club.is_null() && manager_club.nation() == person_club.nation();
    let manager_club_in_person_nation = !manager_club.is_null() && manager_club.nation() == person.nation();
    let club_in_manager_nation = !person_club.is_null() && person_club.nation() == manager.nation();
    let same_division = !manager_club.is_null() && !person_club.is_null() && manager_club.division() == person_club.division();
    if fog_is_famous(person) {
        covered!("sx10_3 famous person kept");
        return FOG_FULLY_KNOWN;
    }
    if !person_club.is_null() && person_club == manager_club {
        return FOG_FULLY_KNOWN;
    }
    if !person.national_job().is_null() && person.national_job() == manager.national_job() {
        return FOG_FULLY_KNOWN;
    }
    if fog_contract_club(person) == manager.club_job().addr() {
        return FOG_FULLY_KNOWN;
    }
    if !person.club_job().is_null() && fog_linked_club(person.club_job().addr()) == manager.club_job().addr() {
        return FOG_FULLY_KNOWN;
    }
    if fog_contract_club(person) != 0 {
        let club = fog_contract_club(person);
        if fog_linked_club(club) == manager.club_job().addr() {
            return FOG_FULLY_KNOWN;
        }
    }
    if same_division {
        return 2; // sx10_3: the division floor
    }
    if clubs_same_nation {
        if let Some(level) = fog_minimum_by_standing(person) {
            return level;
        }
    }
    if same_nation || manager_club_in_person_nation {
        if let Some(level) = fog_minimum_by_standing(person) {
            return level;
        }
    }
    if club_in_manager_nation {
        return fog_reputation_above(person, FOG_KNOWN_REPUTATION) as u8;
    }
    0
}

compare_value!(compare_fog_of_war_minimum_knowledge_level, fog_of_war_minimum_knowledge_level, %%I5%%, (manager: usize, person: usize) -> u8);
