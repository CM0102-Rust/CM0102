// ---------- batch 308: virtual players for a match: topping up and releasing the squads (virtual_staff.cpp) ----------

/// Exe addresses the fixture side of the virtual staff uses that have no name in exe_fns.rs.
pub(crate) mod virtual_fixture_fns {
    pub(crate) const GET_ID_BLOCK: usize = 0x90e760; // VIRTUAL_STAFF::get_id_block
    pub(crate) const RELOCATE_VIRTUAL_PLAYERS: usize = 0x90e9a0; // VIRTUAL_STAFF::relocate_virtual_players
    pub(crate) const TOP_UP_CLUB_SQUAD: usize = 0x90dae0; // VIRTUAL_STAFF::top_up_club_squad
    pub(crate) const VIRTUAL_STAFF_CPP: usize = 0xad439c; // "E:\dev\CM3\cm3 00-01\cm3\code\virtual_staff.cpp"
    /// The fixture list of the VIRTUAL_STAFF object: an inline LLIST of 6-byte nodes (fixture pointer, id block slot).
    pub(crate) const FIXTURE_LIST: usize = 0x5c;
    /// The 17 viewed club slots of the VIRTUAL_STAFF object (one per human manager slot and one spare).
    pub(crate) const VIEWED_CLUBS: usize = 0x18;
    /// The fewest players a side may have once topped up before the game reports an error and starts again.
    pub(crate) const MIN_SQUAD_AFTER_TOP_UP: u8 = 15;
}

/// The first staff and player ids of the block of 25 virtual records of a fixture side or of a viewed club slot
/// (`VIRTUAL_STAFF::get_id_block`, still the exe's): `slot` is passed as the original's whole register.
unsafe fn virtual_id_block(this: usize, slot: u32, is_fixture: u32, is_home: u32) -> (u32, u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32, *mut u32) = core::mem::transmute(virtual_fixture_fns::GET_ID_BLOCK);
    let (mut staff_id, mut player_id) = (0u32, 0u32);
    f(this, slot, is_fixture, is_home, &mut staff_id, &mut player_id);
    (staff_id, player_id)
}

/// The fixture's node in the VIRTUAL_STAFF fixture list, or 0: the cursor goes back to the head first.
unsafe fn virtual_fixture_node(this: usize, fixture: usize) -> usize {
    let list = Llist(this + virtual_fixture_fns::FIXTURE_LIST);
    list.set_cursor(list.head());
    loop {
        let node = exe::llist_get_next(list);
        if node == 0 || read_usize(node) == fixture {
            return node;
        }
    }
}

/// Takes every virtual player (staff id at or above the staff count) out of the club's 50 squad slots, and out of
/// its team selection twice (`exe::tactics_remove_player_from_selection` with 1 and then 0).
unsafe fn drop_club_virtual_players(club: Club) {
    for i in 0..50 {
        let s = club.squad(i);
        if s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT) {
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 1, 1);
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 0, 1);
            club.set_squad(i, 0);
        }
    }
}

/// Puts a fixture on the virtual staff list and tops up both sides' squads with virtual players.
///
/// `VIRTUAL_STAFF::fill_fixture_squads(fixture, slot, no_check)`. A fixture already on the list (`FIXTURE_LIST`)
/// returns at once. The slot of the id block: when `slot` is -1, a human side gets the slot of its manager (the
/// staff count's low word minus the manager's id (`exe::database_get_club_manager`) minus 1, the home side first,
/// `exe::human_manager_is_human_club` with 0, 1), otherwise -2. Each side's block (`get_id_block` with 1 and home
/// 1 / away 0) feeds `top_up_club_squad` (club, the block's staff and player ids, the fixture, and 1 when the side's
/// `fixture::HOME_SIDE_MARK` / `AWAY_SIDE_MARK` is 0); then the fixture enters the list with its slot. Unless
/// `no_check`, a side left with fewer than 15 players in its squad shows the game's error box (line 0xc3), the
/// fixture is released (`release_fixture`) and filled again with the same slot and no check.
/// Original: `VIRTUAL_STAFF::fill_fixture_squads` (0x90d2f0, virtual_staff.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn fill_fixture_squads(this: usize, fixture: usize, slot: u32, no_check: u8) {
    use virtual_fixture_fns as vf;
    if virtual_fixture_node(this, fixture) != 0 {
        return;
    }
    let home = Club(read_usize(fixture + fixture::HOME_CLUB));
    let away = Club(read_usize(fixture + fixture::AWAY_CLUB));
    let mut slot = slot;
    if slot as i16 == -1 {
        // the original rewrites the low word (staff count - manager id) and then decrements the whole register
        let human_slot = |club: Club, slot: u32| {
            let id = read_u16(exe::database_get_club_manager(club));
            ((slot & 0xffff_0000) | read_u16(G_STAFF_COUNT).wrapping_sub(id) as u32).wrapping_sub(1)
        };
        if exe::human_manager_is_human_club(home, 0, 1, 0, 0) != 0 {
            covered!("mwg_1 human home side");
            slot = human_slot(home, slot);
        } else if exe::human_manager_is_human_club(away, 0, 1, 0, 0) != 0 {
            slot = human_slot(away, slot);
        } else {
            slot = 0xffff_fffe;
        }
    }
    let top_up: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) = core::mem::transmute(vf::TOP_UP_CLUB_SQUAD);
    let (staff_id, player_id) = virtual_id_block(this, slot, 1, 1);
    top_up(this, home.addr(), staff_id, player_id, fixture, (read_u8(fixture + fixture::HOME_SIDE_MARK) == 0) as u32);
    let (staff_id, player_id) = virtual_id_block(this, slot, 1, 0);
    covered!("mwg_1 both sides topped up");
    top_up(this, away.addr(), staff_id, player_id, fixture, (read_u8(fixture + fixture::AWAY_SIDE_MARK) == 0) as u32); // mwg1: the away side's top-up
    let mut node = [0u8; 6];
    node[..4].copy_from_slice(&(fixture as u32).to_le_bytes());
    node[4..].copy_from_slice(&(slot as u16).to_le_bytes());
    exe::llist_add_tail(this + vf::FIXTURE_LIST, node.as_mut_ptr() as usize, 6);
    if no_check != 0 {
        return;
    }
    let filled = |club: Club| (0..50).filter(|&i| club.squad(i) != 0).count() as u8;
    if filled(home) < vf::MIN_SQUAD_AFTER_TOP_UP || filled(away) < vf::MIN_SQUAD_AFTER_TOP_UP {
        covered!("mwg_1 short squad after the top-up");
        show_error_at(vf::VIRTUAL_STAFF_CPP, 0xc3);
        exe::virtual_staff_release_fixture(this, fixture);
        exe::virtual_staff_fill_fixture_squads(this, fixture, slot as i32, 1);
    }
}

/// Comparer for `fill_fixture_squads`.
///
/// The fixture goes on a list (a new node) and the virtual players are created with the generator: it cannot run
/// twice. Only the original runs, and what it guarantees is checked: the fixture is on the list afterwards.
pub(crate) unsafe extern "thiscall" fn compare_fill_fixture_squads(this: usize, fixture: usize, slot: u32, no_check: u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u8)>(IDX_FILL_FIXTURE_SQUADS)(this, fixture, slot, no_check);
    let listed = virtual_fixture_node(this, fixture) != 0;
    SCOREBOARD.record(listed, || format!("fill_fixture_squads(fixture {:x}, slot {:#x}, no_check {}): the fixture is not on the list", fixture, slot, no_check));
}

/// Takes a finished fixture's virtual players away and the fixture off the virtual staff list.
///
/// `VIRTUAL_STAFF::release_fixture(fixture)`. Over the 17 viewed club slots (`VIEWED_CLUBS`), the first slot holding
/// the home club (and then, in another slot, the away club) keeps that club's virtual players: they move to the slot's
/// block (`get_id_block` with 0, 0; `relocate_virtual_players`). A side without a slot loses them: every squad slot
/// whose person has a staff id at or above the staff count is emptied and the person taken out of the team selection
/// (`exe::tactics_remove_player_from_selection` with 1, then 0). Last, the fixture's node leaves the list
/// (`exe::llist_remove`).
/// Original: `VIRTUAL_STAFF::release_fixture` (0x90d6b0, virtual_staff.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn release_fixture(this: usize, fixture: usize) {
    use virtual_fixture_fns as vf;
    let home = read_usize(fixture + fixture::HOME_CLUB);
    let away = read_usize(fixture + fixture::AWAY_CLUB);
    let relocate: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(vf::RELOCATE_VIRTUAL_PLAYERS);
    let (mut home_kept, mut away_kept) = (false, false);
    for k in 0..17u32 {
        let viewed = read_usize(this + vf::VIEWED_CLUBS + k as usize * 4);
        let keep = if !home_kept && viewed == home {
            home_kept = true;
            true
        } else if !away_kept && viewed == away {
            away_kept = true;
            true
        } else {
            false
        };
        if keep {
            covered!("mwg_1 viewed club keeps its virtual players");
            let (staff_id, player_id) = virtual_id_block(this, k, 0, 0);
            relocate(this, viewed, staff_id, player_id);
        }
    }
    if !home_kept {
        covered!("mwg_1 side without a viewed slot");
        drop_club_virtual_players(Club(home)); // mwg1: the home side's virtual players leave
    }
    if !away_kept {
        drop_club_virtual_players(Club(away));
    }
    let node = virtual_fixture_node(this, fixture);
    if node != 0 {
        exe::llist_remove(this + vf::FIXTURE_LIST, node);
    }
}

/// Comparer for `release_fixture`.
///
/// The node leaves the list (freed): it cannot run twice. Only the original runs, and what it guarantees is checked:
/// the fixture is no longer on the list.
pub(crate) unsafe extern "thiscall" fn compare_release_fixture(this: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RELEASE_FIXTURE)(this, fixture);
    let listed = virtual_fixture_node(this, fixture) != 0;
    SCOREBOARD.record(!listed, || format!("release_fixture(fixture {:x}): the fixture is still on the list", fixture));
}

/// The position group a virtual player fills, from the player's position ratings (15 or more counts).
///
/// `VIRTUAL_STAFF::get_player_position_group(staff)`: no player data gives 0xff; a goalkeeper (`.goalkeeper()` ≥ 15)
/// 0; a defender (`.defender()` or `.sweeper()` ≥ 15) 1 on the left (`.left_side()`), else 2 on the right
/// (`.right_side()`), else 3 (centre); a midfielder (`.defensive_midfielder()`, `.midfielder()` or
/// `.attacking_midfielder()`) 4 on the left, else 6 on the right, else 5; an attacker (`.attacker()`) 7; anyone else
/// 0xff.
/// Original: `VIRTUAL_STAFF::get_player_position_group` (0x90e020, virtual_staff.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_player_position_group(_this: usize, staff: usize) -> u8 {
    const GOOD: i8 = 15;
    let p = Staff(staff).player();
    if p.is_null() {
        return 0xff;
    }
    if p.goalkeeper() >= GOOD {
        return 0;
    }
    if p.defender() >= GOOD || p.sweeper() >= GOOD {
        covered!("mwg_1 defender");
        return if p.left_side() >= GOOD { 1 } else if p.right_side() >= GOOD { 2 } else { 3 }; // mwg1: the defenders' side
    }
    if p.defensive_midfielder() >= GOOD || p.midfielder() >= GOOD || p.attacking_midfielder() >= GOOD {
        return if p.left_side() >= GOOD { 4 } else if p.right_side() >= GOOD { 6 } else { 5 };
    }
    if p.attacker() >= GOOD { 7 } else { 0xff }
}

compare_value!(compare_get_player_position_group, get_player_position_group, IDX_GET_PLAYER_POSITION_GROUP, (staff: usize) -> u8);

// ---------- batch 308: virtual players: counting a squad and topping it up (virtual_staff.cpp) ----------

/// Exe addresses the squad top-up of the virtual staff uses that have no name in exe_fns.rs.
pub(crate) mod virtual_squad_fns {
    pub(crate) const GET_PLAYER_POSITION_GROUP: usize = 0x90e020; // VIRTUAL_STAFF::get_player_position_group
    pub(crate) const COUNT_AVAILABLE_PLAYERS: usize = 0x90d8f0; // VIRTUAL_STAFF::count_available_players
    pub(crate) const CREATE_VIRTUAL_PLAYER: usize = 0x90e0a0; // VIRTUAL_STAFF::create_virtual_player
    pub(crate) const GET_MAX_SQUAD_SIZE: usize = 0x7754c0; // NationalTeams::get_max_squad_size
    pub(crate) const GET_MIN_SQUAD_SIZE: usize = 0x775520; // NationalTeams::get_min_squad_size
    /// The fewest players of each of the 8 position groups (`get_player_position_group`) a side needs: a club or
    /// main national squad at +0 (4, 4, 4, 8, 4, 8, 4, 8 from the constructor), a B national squad at +8 (6, 4, 4, 12,
    /// 4, 8, 4, 8) (bytes of the VIRTUAL_STAFF object).
    pub(crate) const MIN_BY_GROUP: usize = 0x0;
    /// The fewest players of each position group before a club outside the selected leagues tops up its eligible
    /// count past 20 (2, 2, 2, 4, 2, 4, 2, 4; bytes of the VIRTUAL_STAFF object).
    pub(crate) const MIN_BY_GROUP_LOWER_CLUB: usize = 0x10;
    /// The most virtual players one top-up creates (the size of an id block).
    pub(crate) const BLOCK_SIZE: u8 = 25;
    /// The player record size (70 bytes, `G_PLAYER_DATA_TABLE`).
    pub(crate) const PLAYER_SIZE: usize = 0x46;
}

/// `create_virtual_player` through the exe (the detour runs its comparer in comparison mode).
unsafe fn virtual_create(this: usize, club: Club, group: u8, staff_id: u32, player_id: u32, youngsters: u32, reserve: u32, squad_number: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(virtual_squad_fns::CREATE_VIRTUAL_PLAYER);
    f(this, club.addr(), group as u32, staff_id, player_id, youngsters, reserve, squad_number as u32);
}

/// Counts the players of a club's squad who can play a match, by position group, and drops its leftover virtual
/// players.
///
/// `VIRTUAL_STAFF::count_available_players(club, pass, fixture, &counts, &available, &real, &eligible)`; `pass` is not
/// read. A B national squad is a national team row (`exe::database_is_national_team`) that is not a senior one
/// (`exe::club_is_national_team`). A zeroed 8-byte table (`exe::crt_calloc`) gets one count per position group. Over
/// the 50 squad slots: a virtual player (staff id at or above the staff count) leaves the team selection
/// (`exe::tactics_remove_player_from_selection` with 1, then 0) and the slot; every other person counts as real; one
/// away on international duty today (`exe::nationalteams_get_international_duty_club`), or at a club who is not on
/// its staff (`.club_job()`), stops there; the eligible count takes those marked for the squad (`.playing_squad()` bit
/// 4 in a B squad, bit 1 elsewhere); the badly injured (`exe::injury_get_player_injury_status` 2) and, with a fixture
/// not marked -2, the banned for its competition (`exe::discipline_is_player_banned`) stop; the rest are available
/// and add one to their position group (`get_player_position_group`, none for 0xff). The counts go out through the
/// non-null pointers; without a table pointer the table is freed (the check of 30 available and enough goalkeepers
/// before it ends the same way both ways). Returns 1.
/// Original: `VIRTUAL_STAFF::count_available_players` (0x90d8f0, virtual_staff.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn count_available_players(this: usize, club: usize, _pass: u32, fixture: usize, out_counts: *mut usize, out_available: *mut u8, out_real: *mut u8, out_eligible: *mut u8) -> u32 {
    let c = Club(club);
    let b_squad = exe::database_is_national_team(c) != 0 && exe::club_is_national_team(c) == 0;
    let counts = exe::crt_calloc(8, 1);
    let group: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(virtual_squad_fns::GET_PLAYER_POSITION_GROUP);
    let (mut available, mut real, mut eligible) = (0u8, 0u8, 0u8);
    for i in 0..50 {
        let s = c.squad(i);
        if s == 0 {
            continue;
        }
        if read_i32(s) >= read_i32(G_STAFF_COUNT) {
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), c, s, 1, 1);
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), c, s, 0, 1);
            c.set_squad(i, 0);
            continue;
        }
        real = real.wrapping_add(1);
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), s, G_CURRENT_DATE, c, 0, 0) != 0 {
            continue;
        }
        let person = Staff(s);
        if exe::database_is_national_team(c) == 0 && person.club_job() != c {
            continue;
        }
        let mark = person.playing_squad();
        if (b_squad && mark & 4 != 0) || (!b_squad && mark & 1 != 0) {
            eligible = eligible.wrapping_add(1);
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), s) == 2 {
            covered!("mwg_2 injured player");
            continue; // mwg2: the injured are not available
        }
        if fixture != 0 && read_i32(fixture + fixture::COMP_ID) != -2 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), s, read_usize(fixture + fixture::COMPETITION), 1, 0) != 0 {
            covered!("mwg_2 banned player");
            continue;
        }
        let g = group(this, s);
        if g != 0xff {
            let k = counts.wrapping_add(g as i8 as isize as usize);
            write_u8(k, read_u8(k).wrapping_add(1));
        }
        available = available.wrapping_add(1);
    }
    if !out_counts.is_null() {
        *out_counts = counts;
    }
    if !out_available.is_null() {
        *out_available = available;
    }
    if !out_real.is_null() {
        *out_real = real;
    }
    if !out_eligible.is_null() {
        *out_eligible = eligible;
    }
    if out_counts.is_null() {
        exe::crt_free(counts);
    }
    1
}

/// Comparer for `count_available_players`.
///
/// The original runs first (it drops the virtual players from the squad and the selection) with its own outputs; the
/// copy then counts the same squad (the real players are untouched) with its own: the three counts and the 8 group
/// counts must match. The caller gets the original's outputs, and its table is freed when the caller asked for none.
pub(crate) unsafe extern "thiscall" fn compare_count_available_players(this: usize, club: usize, pass: u32, fixture: usize, out_counts: *mut usize, out_available: *mut u8, out_real: *mut u8, out_eligible: *mut u8) -> u32 {
    type F = unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32;
    let (mut ot, mut oa, mut or, mut oe) = (0usize, 0u8, 0u8, 0u8);
    let r = original_fn::<F>(IDX_COUNT_AVAILABLE_PLAYERS)(this, club, pass, fixture, &mut ot, &mut oa, &mut or, &mut oe);
    let (mut nt, mut na, mut nr, mut ne) = (0usize, 0u8, 0u8, 0u8);
    let n = count_available_players(this, club, pass, fixture, &mut nt, &mut na, &mut nr, &mut ne);
    let table = |t: usize| (0..8).map(|k| read_u8(t + k)).collect::<Vec<u8>>();
    let (to, tn) = (table(ot), table(nt));
    SCOREBOARD.record(r == n && oa == na && or == nr && oe == ne && to == tn, || format!("count_available_players(club {:x}): original {} available {} real {} eligible {} groups {:?}; rust {} available {} real {} eligible {} groups {:?}",
        club, r, oa, or, oe, to, n, na, nr, ne, tn));
    exe::crt_free(nt);
    if !out_counts.is_null() {
        *out_counts = ot;
    } else {
        exe::crt_free(ot);
    }
    if !out_available.is_null() {
        *out_available = oa;
    }
    if !out_real.is_null() {
        *out_real = or;
    }
    if !out_eligible.is_null() {
        *out_eligible = oe;
    }
    r
}

/// Creates virtual players for the position groups where a club (or national squad) is short, until it has enough.
///
/// `VIRTUAL_STAFF::top_up_club_squad(club, staff_id, player_id, fixture, pass)`. First the seed to put back at the
/// end (`exe::utils_random(30000)` + 1). A club without a nation (`.nation()`) or one of the MLS All Stars sides
/// (`G_KEY_CLUB_MLS_ALL_STARS_EAST`/`WEST`) is left alone. The squad is counted (`count_available_players`: the group
/// table, available, real and eligible). The created players are youngsters (`create_virtual_player`: born 17 years before, weaker)
/// at a senior national team, at a club whose nation's league is selected (`nation::LEAGUE_SELECTED`) and whose
/// division has a competition object (`G_COMP_OBJECT_TABLE`), and at a Japanese club whose division is none of the
/// J.League 1 and 2, the Football, University and Administrative Division leagues; a B national squad uses the second row of group minimums
/// (`MIN_BY_GROUP` + 8). The target: a national squad managed by a human (`exe::human_manager_is_human_club` with 1,
/// 1) is left alone; otherwise the main and B squad maxima (`NationalTeams::get_max_squad_size` with 0 and 1) add up
/// to the target, a B squad without a minimum (`get_min_squad_size` with 1 equal to 0) drops the B part; at a senior
/// national team whose nation's team is human, a human who does not manage all national teams
/// (`exe::human_manager_controls_all_national_teams`) leaves it alone, and with no main squad minimum, or more
/// available than it, a nation with more than 500 staff (`.number_staff()`) gets no virtual players (target 0). A
/// club's target is 50, and a computer club needs 20 eligible players before the lower minimums. The reserve flag
/// starts as `exe::database_club_has_reserve_squad`. While the real players are fewer than the target and fewer than
/// 25 were created, each round over the 8 groups (stopping at 50 real players or 25 created): a group under its
/// minimum (`MIN_BY_GROUP`) sets the reserve flag after group 0 when more than 20 are available; the generator is
/// seeded (`exe::crt_srand`) with (players already created for the group + 10000) × (club id + 1) × (group + 1000), so
/// a club always gets the same virtual players; a national squad creates a main squad player (`create_virtual_player`,
/// not reserve, one more eligible) while the main maximum is above the eligible, else at a B squad with a B maximum
/// above available − eligible a reserve one (and ends everything when available − eligible passes 20); a club creates
/// a normal player (one more eligible), except when its division is a selected league or it already has its group
/// minimum (`MIN_BY_GROUP_LOWER_CLUB`) and 20 eligible, and it has the computer's 20 eligible: then a reserve player
/// when the reserve flag is set, or none with 20 eligible. Each step moves the staff and player ids, the group count,
/// available and created on (the squad numbers run from 51); a round without a short group ends it. At the end the
/// group table is freed and the generator seeded again with the seed of the start.
/// Original: `VIRTUAL_STAFF::top_up_club_squad` (0x90dae0, virtual_staff.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn top_up_club_squad(this: usize, club: usize, staff_id: u32, player_id: u32, fixture: usize, pass: u32) {
    use virtual_squad_fns as vs;
    let c = Club(club);
    let seed = exe::utils_random(30000).wrapping_add(1);
    if c.nation().is_null() {
        return;
    }
    if read_u32(club) == read_u32(G_KEY_CLUB_MLS_ALL_STARS_WEST) || read_u32(club) == read_u32(G_KEY_CLUB_MLS_ALL_STARS_EAST) {
        return;
    }
    let count: unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32 = core::mem::transmute(vs::COUNT_AVAILABLE_PLAYERS);
    let (mut counts, mut available, mut real, mut eligible) = (0usize, 0u8, 0u8, 0u8);
    if count(this, club, pass, fixture, &mut counts, &mut available, &mut real, &mut eligible) == 0 {
        if counts != 0 {
            exe::crt_free(counts);
        }
        return;
    }
    let squad_size: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(vs::GET_MAX_SQUAD_SIZE);
    let min_squad: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(vs::GET_MIN_SQUAD_SIZE);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    let (mut b_squad, mut youngsters) = (0usize, 0u32);
    if exe::database_is_national_team(c) != 0 {
        if exe::club_is_national_team(c) != 0 {
            youngsters = 1;
        } else {
            b_squad = 1;
        }
    } else {
        let n = c.nation();
        let d = c.division();
        let comp_object = |d: Competition| read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) != 0;
        let japanese_lower = |d: Competition| {
            let id = read_u32(d.addr());
            ![KEY_CLUB_COMP_JAPANESE_ADMINISTRATIVE_DIVISION, KEY_CLUB_COMP_JAPANESE_UNIVERSITY_LEAGUE, KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1]
                .iter()
                .any(|&k| id == read_u32(k))
        };
        if n.league_selected() != 0 && !d.is_null() && comp_object(d) {
            covered!("mwg_2 selected league club");
            youngsters = 1; // mwg2: a selected league's club gets young virtual players
        } else if read_u32(n.addr()) == read_u32(KEY_NATION_JAPAN) && !d.is_null() && japanese_lower(d) {
            covered!("mwg_2 japanese club outside the named leagues");
            youngsters = 1;
        }
    }
    let (mut max_main, mut max_b, mut min_eligible) = (0xffu8, 0xffu8, 0xffu8);
    let mut target: u8;
    if exe::database_is_national_team(c) != 0 {
        if exe::human_manager_is_human_club(c, 1, 1, 0, 0) != 0 {
            return;
        }
        max_main = squad_size(national_teams, club, 0);
        max_b = squad_size(national_teams, club, 1);
        target = max_main.wrapping_add(max_b);
        if exe::club_is_national_team(c) == 0 {
            if min_squad(national_teams, club, 1) == 0 {
                max_b = 0;
                target = max_main;
            }
        } else {
            let team = exe::database_get_national_team(c.nation());
            if exe::human_manager_is_human_club(team, 0, 1, 0, 0) != 0 {
                let mut all = 0u32;
                if exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), 0usize, &mut all) != 0 || all == 0 {
                    return;
                }
                if pass == 0 && exe::club_is_national_team(c) == 0 {
                    return;
                }
            }
            let min = min_squad(national_teams, club, 0);
            if (min == 0 || available as i8 > min as i8) && c.nation().number_staff() > 500 {
                covered!("mwg_2 nation with enough staff");
                max_main = 0;
                max_b = 0;
                target = 0;
            }
        }
    } else {
        target = 50;
        if exe::human_manager_is_human_club(c, 1, 1, 0, 0) == 0 {
            min_eligible = 20;
        }
    }
    let mut reserve = exe::database_club_has_reserve_squad(c);
    if (real as i8) < (target as i8) {
        let mut created = [0u8; 8];
        let (mut staff_id, mut player_id) = (staff_id, player_id);
        let (mut squad_number, mut made) = (0x33u8, 0u8);
        'rounds: while (made as i8) < vs::BLOCK_SIZE as i8 {
            let mut progress = false;
            for g in 0..8u8 {
                if real as i8 >= 50 || made as i8 >= vs::BLOCK_SIZE as i8 {
                    break;
                }
                let have = read_i8(counts + g as usize);
                if have >= read_i8(this + vs::MIN_BY_GROUP + b_squad * 8 + g as usize) {
                    continue;
                }
                covered!("mwg_2 group below its minimum");
                if g != 0 && available as i8 > 20 && reserve == 0 {
                    reserve = 1;
                }
                let before = created[g as usize];
                let seed_g = (before as i8 as i32 + 10000).wrapping_mul(read_i32(club).wrapping_add(1)).wrapping_mul(g as i32 + 1000);
                exe::crt_srand(seed_g as u32); // mwg2: the seed of the club's virtual players
                if exe::database_is_national_team(c) != 0 {
                    if (max_main as i8) > (eligible as i8) {
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 0, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        eligible = eligible.wrapping_add(1);
                    } else if exe::club_is_national_team(c) == 0 && max_b != 0 && (max_b as i8 as i32) > (available as i8 as i32 - eligible as i8 as i32) {
                        covered!("mwg_2 national b squad player");
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 1, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        if available as i8 as i32 - eligible as i8 as i32 > 20 {
                            break 'rounds;
                        }
                    }
                } else {
                    let d = c.division();
                    let lower = d.is_null() || read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) == 0;
                    let normal = if lower && (have < read_i8(this + vs::MIN_BY_GROUP_LOWER_CLUB + g as usize) || (eligible as i8) < 20) {
                        true
                    } else if min_eligible != 0xff && (available as i8) < (min_eligible as i8) {
                        true
                    } else if reserve != 0 {
                        covered!("mwg_2 reserve player");
                        virtual_create(this, c, g, staff_id, player_id, youngsters, reserve, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        false
                    } else {
                        (eligible as i8) < 20
                    };
                    if normal {
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 0, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        eligible = eligible.wrapping_add(1);
                    }
                }
                progress = true;
                write_u8(counts + g as usize, (have as u8).wrapping_add(1));
                staff_id = staff_id.wrapping_add(1);
                player_id = player_id.wrapping_add(1);
                available = available.wrapping_add(1);
                made = made.wrapping_add(1); // mwg2: one more created
                created[g as usize] = before.wrapping_add(1);
            }
            if !progress || (real as i8) >= (target as i8) {
                break;
            }
        }
    }
    exe::crt_free(counts);
    exe::crt_srand(seed as u32);
}

/// Comparer for `top_up_club_squad`.
///
/// Side by side from the same state: the generator (`crate::rng::STATE_REGIONS`, with the CRT seed), the club's 50
/// squad slots and the 25 staff and player records of the id block. The created players (`create_virtual_player`)
/// run their own comparer in the original's pass.
///
/// Before the two passes the original squad count (`count_available_players`) runs once: it updates each player's
/// injury (`0x62c400` -> `0x62c680`, which draws and keeps its result in the injury table) and drops the leftover
/// virtual players, effects outside the compared regions. Without it the first pass made those draws and the second
/// found them done: in the 308th batch's real game, 256 false divergences where only the draw pointer differed.
pub(crate) unsafe extern "thiscall" fn compare_top_up_club_squad(this: usize, club: usize, staff_id: u32, player_id: u32, fixture: usize, pass: u32) {
    {
        type F = unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32;
        let (mut t, mut a, mut r, mut e) = (0usize, 0u8, 0u8, 0u8);
        original_fn::<F>(IDX_COUNT_AVAILABLE_PLAYERS)(this, club, pass, fixture, &mut t, &mut a, &mut r, &mut e);
        if t != 0 {
            exe::crt_free(t);
        }
    }
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((club + club::SQUAD, 50 * 4));
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, virtual_squad_fns::BLOCK_SIZE as usize * staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_squad_fns::PLAYER_SIZE, virtual_squad_fns::BLOCK_SIZE as usize * virtual_squad_fns::PLAYER_SIZE));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(IDX_TOP_UP_CLUB_SQUAD);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, staff_id, player_id, fixture, pass), || {
        crate::rng::without_counting(|| top_up_club_squad(this, club, staff_id, player_id, fixture, pass))
    });
    SCOREBOARD.record(equal, || format!("top_up_club_squad(club {:x}, staff {}, player {}, pass {}): original {:x?} rust {:x?}", club, staff_id, player_id, pass, mo, mr));
}

// ---------- batch 308: creating one virtual player (virtual_staff.cpp) ----------

/// Constants of `create_virtual_player` that have no name elsewhere.
pub(crate) mod virtual_player_consts {
    /// The player record size (70 bytes, `G_PLAYER_DATA_TABLE`).
    pub(crate) const PLAYER_SIZE: usize = 0x46;
    /// First name and second name counts of the name tables (`G_FIRST_NAME_TABLE`, `G_SECOND_NAME_TABLE`).
    pub(crate) const FIRST_NAME_COUNT: usize = 0xae2398;
    pub(crate) const SECOND_NAME_COUNT: usize = 0xae239c;
    /// The name record size (60 bytes).
    pub(crate) const NAME_SIZE: usize = 60;
    /// f64 0.016 and 0.052 (the ability of a club and of a national team from its reputation).
    pub(crate) const F64_0_016: usize = 0x970a80;
    pub(crate) const F64_0_052: usize = 0x970a78;
    /// The Basque clubs whose virtual players get the Basque second nation (`KEY_NATION_PAYS_BASQUE`): Athletic
    /// Bilbao, its B side, Real Sociedad, its B side and Alavés (key ids).
    pub(crate) const BASQUE_CLUBS: [usize; 5] = [0x9d0120, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124];
    /// The most template players looked at for one with the group's positions.
    pub(crate) const TEMPLATE_TRIES: i32 = 1000;
}

/// Whether a template player record plays the position group (`get_player_position_group` numbering; 15 or more,
/// and exactly 20 for a goalkeeper).
unsafe fn template_fits_group(t: PlayerSheet, group: i32) -> bool {
    const GOOD: i8 = 15;
    match group {
        0 => t.goalkeeper() == 20, // mwg3: a template goalkeeper is a perfect one
        1 => t.defender() >= GOOD && t.left_side() >= GOOD,
        2 => t.defender() >= GOOD && t.right_side() >= GOOD,
        3 => t.defender() >= GOOD && t.central() >= GOOD,
        4 => t.midfielder() >= GOOD && t.left_side() >= GOOD,
        5 => t.midfielder() >= GOOD && t.central() >= GOOD,
        6 => t.midfielder() >= GOOD && t.right_side() >= GOOD,
        7 => t.attacker() >= GOOD,
        _ => false,
    }
}

/// Creates one virtual player of a club at the given staff and player record ids and puts him in the squad.
///
/// `VIRTUAL_STAFF::create_virtual_player(club, group, staff_id, player_id, youngster, reserve, squad_number)`. The
/// staff record gets its id and names: a first and a second name drawn for the club's nation
/// (`exe::database_pick_first_name_for_nation` / `_second_`), or from the whole name tables (draw over the counts)
/// when the club has none, and the common name table's first entry. Born on a drawn day (365) of the year 17 years
/// before the current one rounded down to a multiple of 3 for a youngster, or 17 to 30 years before (draw 14)
/// otherwise; the age (`exe::database_get_staff_age`), the club's nation, the Basque second nation at the Basque clubs
/// (`BASQUE_CLUBS`), no caps, empty national dates; at a national team (`exe::database_is_national_team`) the national
/// job 11 there, otherwise the club job 11; joined the club on a drawn day of the year before, contract to a drawn day
/// six years on; wage the club's reputation / 5, no value, adaptability 1 + draw(10), the other personality ratings
/// 10, classification 2, the player record, no preferences or non-player data, squad mark 1 (2 for a reserve), or at
/// a national team 4 (8). The player record is a copy of a template player drawn from the first 30000 records with
/// the group's positions (`template_fits_group`, up to 1000 draws, the last one kept otherwise); its current and
/// potential ability come from the club's reputation: at a club reputation / 25 − 100, above 5000 reputation × 0.016
/// + 20; at a national team reputation × 0.052 − 150, above 5000 reputation / 100 + 40; a youngster and a reserve
/// each lose 30 + draw(41); then the id, no reputations and the squad number when the club uses squad numbers
/// (`exe::squad_manager_club_uses_squad_numbers`). The staff record takes the first free of the 50 squad slots.
/// Original: `VIRTUAL_STAFF::create_virtual_player` (0x90e0a0, virtual_staff.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn create_virtual_player(_this: usize, club: usize, group: u32, staff_id: u32, player_id: u32, youngster: u32, reserve: u32, squad_number: u32) {
    use virtual_player_consts as vp;
    let c = Club(club);
    let cap = read_i32(G_PLAYER_DATA_COUNT).min(30000);
    let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add(staff_id as usize * staff::SIZE));
    let p = PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add(player_id as usize * vp::PLAYER_SIZE));
    let base_year = read_u16(G_CURRENT_DATE_YEAR) as i32 / 3 * 3;
    write_u32(s.addr(), staff_id);
    if !c.nation().is_null() {
        write_usize(s.addr() + staff::FIRST_NAME, exe::database_pick_first_name_for_nation(c.nation()));
        write_usize(s.addr() + staff::SECOND_NAME, exe::database_pick_second_name_for_nation(c.nation()));
    } else {
        covered!("mwg_3 club without a nation");
        let first = scaled_crt_rand(read_i32(vp::FIRST_NAME_COUNT));
        write_usize(s.addr() + staff::FIRST_NAME, read_usize(G_FIRST_NAME_TABLE).wrapping_add((first as usize).wrapping_mul(vp::NAME_SIZE)));
        let second = scaled_crt_rand(read_i32(vp::SECOND_NAME_COUNT));
        write_usize(s.addr() + staff::SECOND_NAME, read_usize(G_SECOND_NAME_TABLE).wrapping_add((second as usize).wrapping_mul(vp::NAME_SIZE)));
    }
    write_u32(s.addr() + staff::COMMON_NAME, read_u32(G_COMMON_NAME_TABLE));
    let mut date = [0u8; 8];
    let mut tmp = [0u8; 8];
    let d = date.as_mut_ptr() as usize;
    let t = tmp.as_mut_ptr() as usize;
    let born = if youngster != 0 {
        let year = base_year - 17;
        exe::new_date(d, scaled_crt_rand(365) as u32, year as u32)
    } else {
        let year = base_year - scaled_crt_rand(14) - 17;
        exe::new_date(d, scaled_crt_rand(365) as u32, year as u32)
    };
    exe::date_operator_3d(s.addr() + staff::DATE_OF_BIRTH, t, born);
    write_i16(s.addr() + staff::AGE, exe::database_get_staff_age(s, G_CURRENT_DATE as *const u16) as i16);
    write_usize(s.addr() + staff::NATION, c.nation().addr());
    let basque = vp::BASQUE_CLUBS.iter().any(|&k| read_u32(club) == read_u32(k));
    let second_nation = if basque {
        covered!("mwg_3 basque club");
        read_usize(G_NATION_TABLE).wrapping_add(read_u32(KEY_NATION_PAYS_BASQUE) as usize * nation::SIZE)
    } else {
        0
    };
    write_usize(s.addr() + staff::SECOND_NATION, second_nation);
    write_u8(s.addr() + staff::INT_APPS, 0);
    write_u8(s.addr() + staff::INT_GOALS, 0);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION, t, empty);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION + 8, t, empty);
    if exe::database_is_national_team(c) != 0 {
        write_usize(s.addr() + staff::CLUB_JOB, 0);
        write_usize(s.addr() + staff::NATIONAL_JOB, club);
        write_u8(s.addr() + staff::JOB_FOR_CLUB, 0);
        write_u8(s.addr() + staff::JOB_FOR_NATION, 0xb);
    } else {
        write_usize(s.addr() + staff::CLUB_JOB, club);
        write_usize(s.addr() + staff::NATIONAL_JOB, 0);
        write_u8(s.addr() + staff::JOB_FOR_CLUB, 0xb);
        write_u8(s.addr() + staff::JOB_FOR_NATION, 0);
    }
    let joined = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year - 1) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_CLUB, t, joined);
    let expires = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year + 6) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_EXPIRES_CLUB, t, expires);
    write_i32(s.addr() + staff::VALUE, 0);
    write_i32(s.addr() + staff::WAGE, c.reputation() as i32 / 5);
    write_u8(s.addr() + staff::ADAPTABILITY, (scaled_crt_rand(10) as u8).wrapping_add(1));
    for f in [staff::AMBITION, staff::DETERMINATION, staff::LOYALTY, staff::PRESSURE, staff::PROFESSIONALISM, staff::SPORTSMANSHIP, staff::TEMPERAMENT] {
        write_u8(s.addr() + f, 10);
    }
    write_u8(s.addr() + staff::CLASSIFICATION, 2);
    write_usize(s.addr() + staff::PLAYER, p.addr());
    write_usize(s.addr() + staff::STAFF_PREFERENCES, 0);
    write_usize(s.addr() + staff::NON_PLAYER, 0);
    write_u8(s.addr() + staff::SQUAD_SELECTED_FOR, 0);
    let national = exe::database_is_national_team(c) != 0;
    let mark = match (national, reserve != 0) {
        (true, true) => 8,
        (true, false) => 4,
        (false, true) => 2,
        (false, false) => 1,
    };
    write_u8(s.addr() + staff::PLAYING_SQUAD, mark);
    let g = group as u8 as i8 as i32;
    let mut tries = 0;
    let template = loop {
        let k = scaled_crt_rand(cap);
        tries += 1;
        let t = PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add((k as usize).wrapping_mul(vp::PLAYER_SIZE)));
        if template_fits_group(t, g) || tries >= vp::TEMPLATE_TRIES {
            break t;
        }
    };
    core::ptr::copy_nonoverlapping(template.addr() as *const u8, p.addr() as *mut u8, vp::PLAYER_SIZE);
    let rep = c.reputation() as i32;
    let mut ability = if exe::database_is_national_team(c) == 0 {
        if rep > 5000 {
            ftol(rep as f64 * read_f64(vp::F64_0_016) + read_f64(rdata::F64_20))
        } else {
            rep / 25 - 100 // mwg3: the ability of a club's virtual player
        }
    } else if rep > 5000 {
        rep / 100 + 40
    } else {
        ftol(rep as f64 * read_f64(vp::F64_0_052) - read_f64(rdata::F64_150))
    };
    if youngster != 0 {
        covered!("mwg_3 youngster");
        ability = ability.wrapping_add(-30 - scaled_crt_rand(41)); // mwg3: the youngster loses ability
    }
    if reserve != 0 {
        ability = ability.wrapping_add(-30 - scaled_crt_rand(41));
    }
    p.set_current_ability(ability as i16);
    p.set_potential_ability(ability as i16);
    write_u32(p.addr(), player_id);
    p.set_home_reputation(0);
    p.set_current_reputation(0);
    p.set_world_reputation(0);
    let number = if exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), c, 0) != 0 { squad_number as u8 } else { 0 };
    write_u8(p.addr() + 4, number);
    if let Some(i) = (0..50).find(|&i| c.squad(i) == 0) {
        c.set_squad(i, s.addr());
    }
}

/// Comparer for `create_virtual_player`.
///
/// Side by side from the same state: the generator (`crate::rng::STATE_REGIONS`, with the CRT seed), the staff and
/// player records being created and the club's 50 squad slots.
pub(crate) unsafe extern "thiscall" fn compare_create_virtual_player(this: usize, club: usize, group: u32, staff_id: u32, player_id: u32, youngster: u32, reserve: u32, squad_number: u32) {
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_player_consts::PLAYER_SIZE, virtual_player_consts::PLAYER_SIZE));
    regions.push((club + club::SQUAD, 50 * 4));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32)>(IDX_CREATE_VIRTUAL_PLAYER);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, group, staff_id, player_id, youngster, reserve, squad_number), || {
        crate::rng::without_counting(|| create_virtual_player(this, club, group, staff_id, player_id, youngster, reserve, squad_number))
    });
    SCOREBOARD.record(equal, || format!("create_virtual_player(club {:x}, group {:#x}, staff {}, player {}, youngster {}, reserve {}, number {}): original {:x?} rust {:x?}",
        club, group, staff_id, player_id, youngster, reserve, squad_number, mo, mr));
}

// ---------- batch 308: the human manager rows of the staff table (Setup.cpp) ----------

/// The Setup.cpp source name the error box shows.
pub(crate) const SETUP_CPP_FILE: usize = 0xa801dc; // "E:\dev\CM3\cm3 00-01\cm3\code\Setup.cpp"

/// The human manager row `k` (0..16) counted back from the end of the staff table: the 16 human managers are the
/// last 16 staff records.
unsafe fn human_manager_row(k: i32) -> Staff {
    Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - 1 - k) as usize * staff::SIZE))
}

/// The current screen slot's manager (`G_SCREEN_RECORDS_MANAGER`, 0x300 bytes per slot, slot
/// `G_SCREEN_MANAGER_CURRENT_SLOT`).
unsafe fn current_screen_manager() -> Staff {
    Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize) as usize))
}

/// The first of the 16 human manager rows that is free (classification 0), from the last staff record back, or 0.
///
/// `find_free_human_manager_row()`. Without the staff table, the game's error box (line 0x39b3) and 0.
/// Original: `find_free_human_manager_row` (0x838c60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_free_human_manager_row() -> usize {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39b3);
        return 0;
    }
    covered!("mwg_4 free row");
    (0..16).map(|k| human_manager_row(k)).find(|s| s.classification() == 0).map_or(0, |s| s.addr()) // mwg4: a free row
}

/// Comparer for `find_free_human_manager_row`.
pub(crate) unsafe extern "C" fn compare_find_free_human_manager_row() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_FIND_FREE_HUMAN_MANAGER_ROW)();
    let n = find_free_human_manager_row();
    SCOREBOARD.record(r == n, || format!("find_free_human_manager_row() original={:#x} rust={:#x}", r, n));
    r
}

/// How many of the 16 human manager rows are in use (classification 1); with `network_only`, only those whose
/// manager is done on the screen manager (`exe::screenmanager_is_manager_done`).
///
/// `count_human_managers(network_only)`. Without the staff table, the game's error box (line 0x39d8) and 0.
/// Original: `count_human_managers` (0x838d10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn count_human_managers(network_only: u32) -> u32 {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39d8);
        return 0;
    }
    covered!("mwg_4 count");
    (0..16)
        .map(|k| human_manager_row(k))
        .filter(|s| s.classification() == 1 && (network_only == 0 || exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, s.addr() as u32) != 0)) // mwg4: a human in use
        .count() as u32
}

/// Comparer for `count_human_managers`.
pub(crate) unsafe extern "C" fn compare_count_human_managers(network_only: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_COUNT_HUMAN_MANAGERS)(network_only);
    let n = count_human_managers(network_only);
    SCOREBOARD.record(r == n, || format!("count_human_managers({}) original={} rust={}", network_only, r, n));
    r
}

/// 1 when the current screen slot has a manager whose row is still being made (classification 5), else 0.
///
/// `is_current_manager_being_created()`.
/// Original: `is_current_manager_being_created` (0x838de0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_current_manager_being_created() -> u32 {
    covered!("mwg_4 being created");
    let s = current_screen_manager();
    (!s.is_null() && s.classification() == 5) as u32 // mwg4: being created
}

/// Comparer for `is_current_manager_being_created`.
pub(crate) unsafe extern "C" fn compare_is_current_manager_being_created() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_IS_CURRENT_MANAGER_BEING_CREATED)();
    let n = is_current_manager_being_created();
    SCOREBOARD.record(r == n, || format!("is_current_manager_being_created() original={} rust={}", r, n));
    r
}

// ---------- batch 308: the year ratings and the month ratings reset (award/year_ratings.cpp, award/month_ratings.cpp) ----------

/// Constants of the year ratings (`YEAR_RATINGS`: +0 pointer to the 21-byte entries, +4 count, +8 capacity).
pub(crate) mod year_ratings_consts {
    /// The entry size: +0 staff id, +4 nation id (−1 none), +8/+0xa/+0xc three weighted rating sums (i16), +0xe
    /// matches (u8), +0xf goals, +0x11 assists, +0x13 cards (yellow + 2 × red) (u16).
    pub(crate) const ENTRY_SIZE: usize = 21;
    /// How many entries the table grows by when full.
    pub(crate) const GROWTH: i32 = 400;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\award\year_ratings.cpp" (the error box).
    pub(crate) const YEAR_RATINGS_CPP: usize = 0xad6a38;
    /// The competition object's weight word (+0x69) for the year ratings, and its type byte (+0x42: 2 and 4 weigh
    /// differently).
    pub(crate) const COMP_WEIGHT: usize = 0x69;
    pub(crate) const COMP_TYPE: usize = 0x42;
}

/// Adds the first eleven of both sides of a match to the year ratings, for the players the filter accepts.
///
/// `YEAR_RATINGS::record_match_ratings(fixture, record, filter)`. Not for a friendly (`fixture::CATEGORY` 1). For each
/// side (the home club `fixture::HOME_CLUB` with the record's first block, the away club with the block
/// `match_record::SIDE_SIZE` on), the first 11 of the 20 rows: an empty one (staff id −1 or position
/// `row_slot::EMPTY`) and a virtual player (staff id at or above `G_STAFF_COUNT`) are skipped; the rest go to
/// `add_player_rating` when `filter(person, fixture)` (cdecl) accepts them.
/// Original: `YEAR_RATINGS::record_match_ratings` (0x9315d0, award/year_ratings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    if read_u8(fixture + fixture::CATEGORY) == 1 {
        return;
    }
    let accepts: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(filter);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x931690usize);
    for side in 0..2usize {
        let club = read_usize(fixture + if side == 0 { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
        for k in 0..20usize {
            let row = record + 0x190 + side * 0x625 + k * match_record_row::SIZE;
            let id = read_i32(row + match_record_row::STAFF_ID);
            if id == -1 || read_u8(row + match_record_row::POSITION) == row_slot::EMPTY || k >= 11 { // mwg5: the first eleven
                continue;
            }
            let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            if person != 0 && read_i32(person) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if accepts(person, fixture) != 0 {
                if side == 1 {
                    covered!("mwg_5 away side rating");
                }
                add(this, row, fixture, club);
            }
        }
    }
}

/// Comparer for `year_record_match_ratings`.
///
/// Side by side over the table's header and its entries plus room for 22 new ones; when the table could grow (less
/// room, or none yet), only the original runs (each added rating has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_YEAR_RECORD_MATCH_RATINGS);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 22 {
        return o(this, fixture, record, filter);
    }
    let regions = [(this, 12), (tab, (n as usize + 22) * year_ratings_consts::ENTRY_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture, record, filter), || year_record_match_ratings(this, fixture, record, filter));
    SCOREBOARD.record(equal, || format!("year_record_match_ratings(fixture {:x}) original={:x?} rust={:x?}", fixture, mo, mr));
}

/// The three weighted rating sums a match adds to a year ratings entry (the +8, +0xa and +0xc words).
///
/// The weight: with the fixture's competition (`fixture::COMPETITION`), the competition's weight word × (club
/// reputation / 500) / 20 + (club reputation − 5000) / 1700, added as bytes and kept between 1 and 20 (without it the
/// original reads a stack byte it never wrote; the copy uses 0 [inference: no competition object then]). Without a
/// competition object for the fixture's id, the match grade × (club reputation / 500, as a byte, × 16 when the
/// nation's reputation is above 7000, × 8 above 4000, × 4 otherwise, / 20, as a byte) gives /20, /60 and /80; with
/// one, the grade × the weight gives /20, /20, /40 for competition type 2, /20 three times for type 4, and /20, /60,
/// /80 otherwise.
unsafe fn year_rating_sums(fixture: usize, club: Club, comp_obj: usize, grade: i32) -> (i8, i8, i8) {
    use year_ratings_consts as yr;
    let mut weight = 0i8;
    let competition = read_usize(fixture + fixture::COMPETITION);
    if competition != 0 {
        let rep = club.reputation() as i32;
        let a = (read_i16(competition + yr::COMP_WEIGHT) as i32).wrapping_mul(rep / 500) / 20;
        let b = (rep - 5000) / 1700;
        weight = (a as u8).wrapping_add(b as u8) as i8;
        weight = weight.clamp(1, 20); // mwg5: the competition weight within 1..20
    }
    let wide = |v: i32| ((v / 20) as i8, (v / 60) as i8, (v / 80) as i8);
    if comp_obj == 0 {
        covered!("mwg_5 fixture without a competition object");
        let rep = club.reputation() as i32;
        let by_rep = (rep / 500) as i8 as i32;
        let nation_rep = club.nation().reputation();
        let scaled = if nation_rep > 7000 { by_rep << 4 } else if nation_rep > 4000 { by_rep << 3 } else { by_rep << 2 };
        return wide(grade * (scaled / 20) as i8 as i32);
    }
    let v = grade * weight as i32;
    match read_i8(comp_obj + yr::COMP_TYPE) {
        2 => ((v / 20) as i8, (v / 20) as i8, (v / 40) as i8),
        4 => ((v / 20) as i8, (v / 20) as i8, (v / 20) as i8),
        _ => wide(v),
    }
}

/// Finds or appends a player's entry in the year ratings and adds a match to it.
///
/// `YEAR_RATINGS::add_player_rating(row, fixture, club)`. The entry of the row's staff id is searched in the table;
/// missing, it is appended (the table grows by 400 entries, `exe::crt_malloc` / `exe::crt_realloc`; without memory the
/// game's error box, line 0x113, and nothing more) with the person's nation id (`staff::NATION`, −1 none) and zeroed
/// sums. Then the three weighted rating sums (`year_rating_sums`), one more match, the row's goals and assists, and
/// yellow cards + 2 × red cards.
/// Original: `YEAR_RATINGS::add_player_rating` (0x931690, award/year_ratings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_add_player_rating(this: usize, row: usize, fixture: usize, club: usize) {
    use year_ratings_consts as yr;
    let comp_id = read_i32(fixture + fixture::COMP_ID);
    let comp_obj = if comp_id >= 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp_id as usize * 4) } else { 0 };
    let id = read_i32(row + match_record_row::STAFF_ID);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let n = read_i32(this + 4);
    let mut table = read_usize(this);
    let found = (0..n.max(0) as usize).map(|k| table + k * yr::ENTRY_SIZE).find(|&e| read_i32(e) == id);
    let e = match found {
        Some(e) => e,
        None => {
            if n == read_i32(this + 8) {
                let cap = n.wrapping_add(yr::GROWTH);
                write_i32(this + 8, cap);
                let bytes = (cap as u32).wrapping_mul(yr::ENTRY_SIZE as u32);
                table = if table == 0 { exe::crt_malloc(bytes) } else { exe::crt_realloc(table, bytes) };
                write_usize(this, table);
                if table == 0 {
                    show_error_at(yr::YEAR_RATINGS_CPP, 0x113);
                    return;
                }
            }
            covered!("mwg_5 new player in the year");
            let e = table + n as usize * yr::ENTRY_SIZE;
            write_i32(e, id);
            let nation = person.nation();
            write_i32(e + 4, if nation.is_null() { -1 } else { read_i32(nation.addr()) });
            for k in [8, 0xa, 0xc, 0xf, 0x11, 0x13] {
                write_u16(e + k, 0);
            }
            write_u8(e + 0xe, 0);
            write_i32(this + 4, n.wrapping_add(1));
            e
        }
    };
    let grade = read_u8(row + match_record_row::MATCH_GRADE) as i32;
    let (a, b, c) = year_rating_sums(fixture, Club(club), comp_obj, grade);
    let add16 = |k: usize, v: u16| write_u16(e + k, read_u16(e + k).wrapping_add(v));
    add16(8, a as i16 as u16);
    add16(0xa, b as i16 as u16);
    add16(0xc, c as i16 as u16);
    write_u8(e + 0xe, read_u8(e + 0xe).wrapping_add(1));
    add16(0xf, read_u8(row + match_record_row::GOALS) as u16);
    add16(0x11, read_u8(row + match_record_row::ASSISTS) as u16);
    add16(0x13, read_u8(row + match_record_row::YELLOW_CARDS) as u16 + 2 * read_u8(row + match_record_row::RED_CARD) as u16); // mwg5: a red card counts twice
}

/// Comparer for `year_add_player_rating`.
///
/// Side by side over the header and the entries (one more when it appends with room); when the table grows, only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_year_add_player_rating(this: usize, row: usize, fixture: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_YEAR_ADD_PLAYER_RATING);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    let size = year_ratings_consts::ENTRY_SIZE;
    let found = tab != 0 && (0..n.max(0) as usize).any(|k| read_i32(tab + k * size) == read_i32(row));
    if tab == 0 || n < 0 || (n >= cap && !found) {
        return o(this, row, fixture, club);
    }
    let slots = if n < cap { n as usize + 1 } else { n as usize };
    let regions = [(this, 12), (tab, slots * size)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, row, fixture, club), || year_add_player_rating(this, row, fixture, club));
    SCOREBOARD.record(equal, || format!("year_add_player_rating(person {}, fixture {:x}) original={:x?} rust={:x?}", read_i32(row), fixture, mo, mr));
}

/// Empties the month ratings for a new month.
///
/// `MONTH_RATINGS::reset(month, year)`: the entry table is freed (`exe::crt_free`) and cleared, the count and the
/// capacity become 0, and `month_ratings_fields::MONTH` / `YEAR` the new month and year.
/// Original: `MONTH_RATINGS::reset` (0x767f60, award/month_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn month_ratings_reset(this: usize, month: u8, year: i16) {
    covered!("mwg_5 month reset");
    let table = read_usize(this); // mwg5: the old month's table
    if table != 0 {
        exe::crt_free(table);
        write_usize(this, 0);
    }
    write_u32(this + 8, 0);
    write_u32(this + 4, 0);
    write_u8(this + month_ratings_fields::MONTH, month);
    write_i16(this + month_ratings_fields::YEAR, year); // mwg5: the new month's year
}

/// Comparer for `month_ratings_reset`.
///
/// The table is freed: it cannot run twice. Only the original runs, and what it guarantees is checked: an empty table
/// with the month and year given.
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_reset(this: usize, month: u8, year: i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, u8, i16)>(IDX_MONTH_RATINGS_RESET)(this, month, year);
    let ok = read_usize(this) == 0 && read_u32(this + 4) == 0 && read_u32(this + 8) == 0
        && read_u8(this + month_ratings_fields::MONTH) == month && read_i16(this + month_ratings_fields::YEAR) == year;
    SCOREBOARD.record(ok, || format!("month_ratings_reset({}, {}): table {:x} count {} capacity {} month {} year {}",
        month, year, read_usize(this), read_u32(this + 4), read_u32(this + 8), read_u8(this + month_ratings_fields::MONTH), read_i16(this + month_ratings_fields::YEAR)));
}

// ---------- batch 308: the award manager: a nation's awards and the award history files (award/award_manager.cpp) ----------

/// Fields of the AWARD_MANAGER object and of its nation award objects.
pub(crate) mod award_manager_consts {
    /// The two FILE_LLISTs of the award history: year awards (+0) and team awards (+4).
    pub(crate) const YEAR_AWARD_FILE: usize = 0x0;
    pub(crate) const TEAM_AWARD_FILE: usize = 0x4;
    /// The 30-slot table of nation award objects.
    pub(crate) const NATION_AWARDS: usize = 0x8;
    pub(crate) const NATION_AWARD_SLOTS: usize = 30;
    /// A nation award object's nation (+0xe) and its kind byte (+0x12, 1 = a nation's awards).
    pub(crate) const AWARDS_NATION: usize = 0xe;
    pub(crate) const AWARDS_KIND: usize = 0x12;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_manager.cpp" (the error box).
    pub(crate) const AWARD_MANAGER_CPP: usize = 0x987e24;
}

/// Appends a year award history record to the year award file list.
///
/// `AWARD_MANAGER::add_year_award_record(record, first, last)`: `exe::file_llist_add_node` on the first file list
/// (`YEAR_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_year_award_record` (0x4142c0, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::YEAR_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_year_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(IDX_AWARD_MANAGER_ADD_YEAR_AWARD_RECORD)(this, record, first, last)
}

/// Appends a team award history record to the team award file list.
///
/// `AWARD_MANAGER::add_team_award_record(record, first, last)`: `exe::file_llist_add_node` on the second file list
/// (`TEAM_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_team_award_record` (0x414300, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::TEAM_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_team_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(IDX_AWARD_MANAGER_ADD_TEAM_AWARD_RECORD)(this, record, first, last)
}

/// The nation award object of a nation: the first of the 30 slots (`NATION_AWARDS`) holding one of kind 1 for that
/// nation, or 0.
///
/// `AWARD_MANAGER::find_nation_awards(nation)`. Without a nation, the game's error box (line 0x2b3) and 0.
/// Original: `AWARD_MANAGER::find_nation_awards` (0x414340, award/award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_nation_awards(this: usize, nation: usize) -> usize {
    use award_manager_consts as am;
    if nation == 0 {
        show_error_at(am::AWARD_MANAGER_CPP, 0x2b3);
        return 0;
    }
    covered!("mwg_6 nation awards");
    let table = read_usize(this + am::NATION_AWARDS);
    (0..am::NATION_AWARD_SLOTS)
        .map(|i| read_usize(table + i * 4))
        .find(|&e| e != 0 && read_u8(e + am::AWARDS_KIND) == 1 && read_usize(e + am::AWARDS_NATION) == nation) // mwg6: the nation's awards
        .unwrap_or(0)
}

compare_value!(compare_find_nation_awards, find_nation_awards, IDX_FIND_NATION_AWARDS, (nation: usize) -> usize);

// ---------- batch 308: the hall of fame entries of the human managers (hall_of_fame.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_CALENDAR_YEAR_ENTRY);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(IDX_ADD_LEAGUE_ENTRY);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_TRY_ADD_ENTRY);
    let regions = [(this, hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, table, staff, score), || try_add_entry(this, table, staff, score));
    SCOREBOARD.record(equal && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("try_add_entry(table {}, staff {:x}, score {}) original={} rust={}, tables differ from byte {:#x}", table as u8 as i8, staff, score, r, n, k)
    });
    r
}

// ---------- batch 308: comparing match records, a duration in words and a new loan offer (record_utils.cpp, transfer_offer.cpp) ----------

/// Constants of the record utilities.
pub(crate) mod record_utils_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\record_utils.cpp" (the error box).
    pub(crate) const RECORD_UTILS_CPP: usize = 0xa7a088;
    /// A match record's home goals (+0x19) and away goals (+0x1a), signed bytes.
    pub(crate) const HOME_GOALS: usize = 0x19;
    pub(crate) const AWAY_GOALS: usize = 0x1a;
    /// The duration texts (translated by `FN_LANGLIB_SPRINTF`): approximate, then exact.
    pub(crate) const ONE_DAY: usize = 0xa7a1d0; // "one day"
    pub(crate) const A_FEW_DAYS: usize = 0xa7a1c4; // "a few days"
    pub(crate) const ABOUT_A_WEEK: usize = 0xa7a1b4; // "about a week"
    pub(crate) const ABOUT_TEN_DAYS: usize = 0xa7a1a4; // "about ten days"
    pub(crate) const ABOUT_N_WEEKS: usize = 0xa7a188; // "about <%d - number> weeks"
    pub(crate) const ABOUT_A_MONTH: usize = 0xa7a178; // "about a month"
    pub(crate) const ABOUT_N_MONTHS: usize = 0xa7a15c; // "about <%d - number> months"
    pub(crate) const ABOUT_A_YEAR: usize = 0xa7a14c; // "about a year"
    pub(crate) const ABOUT_18_MONTHS: usize = 0xa7a13c; // "about 18 months"
    pub(crate) const ABOUT_2_YEARS: usize = 0xa7a12c; // "about 2 years"
    pub(crate) const ABOUT_N_YEARS: usize = 0xa7a110; // "about <%d - number> years"
    pub(crate) const EXACT_1_DAY: usize = 0xa7a108; // "1 day"
    pub(crate) const EXACT_N_DAYS: usize = 0xa7a0f4; // "<%d - number> days"
    pub(crate) const EXACT_1_WEEK: usize = 0xa7a0ec; // "1 week"
    pub(crate) const EXACT_N_WEEKS: usize = 0xa7a0d8; // "<%d - number> weeks"
    pub(crate) const EXACT_1_MONTH: usize = 0xa7a0d0; // "1 month"
    pub(crate) const EXACT_N_MONTHS: usize = 0xa7a0b8; // "<%d - number> months"
    /// "E:\dev\CM3\cm3 00-01\cm3\code\transfer_offer.cpp" (the error box).
    pub(crate) const TRANSFER_OFFER_CPP: usize = 0xa9e318;
}

/// Whether the first match record has more goals in all than the second.
///
/// `match_record_has_more_goals(a, b)`. A missing record shows the game's error box (line 0x107) and gives 0. When
/// a's home goals plus b's away goals make 0 the answer is 0 (the original mixes the two records in this check: a
/// defect, kept); otherwise a's home + away goals above b's.
/// Original: `match_record_has_more_goals` (0x7e4be0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_record_has_more_goals(a: usize, b: usize) -> u32 {
    use record_utils_consts as ru;
    if a == 0 || b == 0 {
        show_error_at(ru::RECORD_UTILS_CPP, 0x107);
        return 0;
    }
    let a_home = read_i8(a + ru::HOME_GOALS) as i32;
    let b_away = read_i8(b + ru::AWAY_GOALS) as i32;
    if a_home + b_away == 0 {
        covered!("mwg_8 no goals");
        return 0;
    }
    covered!("mwg_8 goals compared");
    let b_total = read_i8(b + ru::HOME_GOALS) as i32 + b_away;
    let a_total = read_i8(a + ru::AWAY_GOALS) as i32 + a_home;
    (a_total > b_total) as u32 // mwg8: more goals
}

/// Comparer for `match_record_has_more_goals`.
pub(crate) unsafe extern "C" fn compare_match_record_has_more_goals(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_HAS_MORE_GOALS)(a, b);
    let n = match_record_has_more_goals(a, b);
    SCOREBOARD.record(r == n, || format!("match_record_has_more_goals({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Writes a number of days as words: approximate ("about a week") or exact ("3 weeks").
///
/// `format_duration_text(days, out, size, approximate)`. A buffer under 12 bytes shows the game's error box (line
/// 0x288) and gives 0; otherwise 1. Under one day the text is empty. Approximate: 1 "one day", under 6 "a few days",
/// under 9 "about a week", under 14 "about ten days", under 28 "about N weeks" (days / 7), under 40 "about a month",
/// under 365 "about N months" (days / 30 + 1), under 450 "about a year", under 630 "about 18 months", under 910 "about
/// 2 years", otherwise "about N years" (days / 365). Exact: under 14 "1 day" or "N days", under 43 "1 week" or "N
/// weeks" (days / 7), otherwise "1 month" or "N months" ((days − 15) / 30 + 1). The texts are translated
/// (`FN_LANGLIB_SPRINTF`).
/// Original: `format_duration_text` (0x7e5780, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn format_duration_text(days: i32, out: usize, size: i32, approximate: u32) -> u32 {
    use record_utils_consts as ru;
    if size < 12 {
        show_error_at(ru::RECORD_UTILS_CPP, 0x288);
        return 0;
    }
    let write: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if days < 1 {
        write_u8(out, 0);
        return 1;
    }
    if approximate != 0 {
        covered!("mwg_8 approximate duration");
        match days {
            1 => write(out, ru::ONE_DAY),
            2..=5 => write(out, ru::A_FEW_DAYS),
            6..=8 => write(out, ru::ABOUT_A_WEEK),
            9..=13 => write(out, ru::ABOUT_TEN_DAYS),
            14..=27 => write(out, ru::ABOUT_N_WEEKS, days / 7), // mwg8: about N weeks
            28..=39 => write(out, ru::ABOUT_A_MONTH),
            40..=364 => write(out, ru::ABOUT_N_MONTHS, days / 30 + 1),
            365..=449 => write(out, ru::ABOUT_A_YEAR),
            450..=629 => write(out, ru::ABOUT_18_MONTHS),
            630..=909 => write(out, ru::ABOUT_2_YEARS),
            _ => write(out, ru::ABOUT_N_YEARS, days / 365),
        };
    } else if days < 14 {
        covered!("mwg_8 exact duration");
        if days == 1 { write(out, ru::EXACT_1_DAY) } else { write(out, ru::EXACT_N_DAYS, days) };
    } else if days < 43 {
        let weeks = days / 7;
        if weeks == 1 { write(out, ru::EXACT_1_WEEK) } else { write(out, ru::EXACT_N_WEEKS, weeks) };
    } else {
        let months = (days - 15) / 30 + 1;
        if months == 1 { write(out, ru::EXACT_1_MONTH) } else { write(out, ru::EXACT_N_MONTHS, months) };
    }
    1
}

/// Comparer for `format_duration_text`: side by side over the output buffer.
pub(crate) unsafe extern "C" fn compare_format_duration_text(days: i32, out: usize, size: i32, approximate: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, i32, u32) -> u32>(IDX_FORMAT_DURATION_TEXT);
    let regions = [(out, size.clamp(0, 256) as usize)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(days, out, size, approximate), || format_duration_text(days, out, size, approximate));
    SCOREBOARD.record(equal && r == n, || format!("format_duration_text({}, approximate {}) original={} {:x?} rust={} {:x?}", days, approximate, r, mo, n, mr));
    r
}

/// Resets an offer record for a loan of `staff` to `club`.
///
/// `TRANSFER_OFFER::init_loan_offer(staff, club)`. Without the person or the club, the game's error box (line 0x485)
/// and nothing more. Otherwise: not known to the player (`offer::PLAYER_AWARE` 0), no exchange list, job 11
/// (`offer::JOB_TYPE`), the buyer's id (`offer::BUYER_CLUB_ID`) and the person's (`offer::TARGET_STAFF_ID`), no fee,
/// the clauses empty with 10 games (`offer::APPEARANCE_GAMES`, `offer::INTERNATIONAL_GAMES`), serial and negotiation
/// −1, no sell-on or instalments, status 0, no terms, response round or pre-contract, a loan (`offer::OFFER_TYPE` 1),
/// the completion timing's low nibble cleared (+0 and the high nibble kept).
/// Original: `TRANSFER_OFFER::init_loan_offer` (0x8e7810, transfer_offer.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn init_loan_offer(this: usize, staff: usize, club: usize) {
    if staff == 0 || club == 0 {
        show_error_at(record_utils_consts::TRANSFER_OFFER_CPP, 0x485);
        return;
    }
    covered!("mwg_8 loan offer");
    write_u8(this + offer::PLAYER_AWARE, 0);
    write_u32(this, 0);
    write_u8(this + offer::JOB_TYPE, 0xb);
    write_usize(this + offer::EXCHANGE_PLAYERS, 0);
    write_u32(this + offer::BUYER_CLUB_ID, read_u32(club));
    write_u32(this + offer::FEE, 0);
    write_u8(this + offer::APPEARANCE_GAMES, 10); // mwg8: the appearance clause's games
    write_u8(this + offer::INTERNATIONAL_GAMES, 10);
    write_u32(this + offer::APPEARANCE_FEE, 0);
    write_i32(this + offer::NEGOTIATION_INDEX, -1);
    write_i32(this + offer::SERIAL, -1);
    write_u32(this + offer::INTERNATIONAL_FEE, 0);
    write_u8(this + offer::SELL_ON_PERCENT, 0);
    write_u8(this + offer::INSTALMENTS, 0);
    write_u8(this + offer::STATUS, 0);
    write_u8(this + offer::TERMS_STATE, 0);
    write_u8(this + offer::RESPONSE_ROUND, 0);
    write_u8(this + offer::PRE_CONTRACT, 0);
    write_u32(this + offer::TARGET_STAFF_ID, read_u32(staff));
    write_u8(this + offer::OFFER_TYPE, 1);
    write_u8(this + offer::COMPLETION_TIMING, read_u8(this + offer::COMPLETION_TIMING) & 0xf0);
}

/// Comparer for `init_loan_offer`: side by side over the offer's 0x41 bytes.
pub(crate) unsafe extern "thiscall" fn compare_init_loan_offer(this: usize, staff: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_INIT_LOAN_OFFER);
    let regions = [(this, offer::INSTALMENTS + 1)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, staff, club), || init_loan_offer(this, staff, club));
    SCOREBOARD.record(equal, || format!("init_loan_offer(staff {:x}, club {:x}) original={:x?} rust={:x?}", staff, club, mo, mr));
}
