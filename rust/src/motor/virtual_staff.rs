use super::*;

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
pub(crate) unsafe extern "thiscall" fn fill_fixture_squads(this: usize, fixture: usize, slot: u32, in_the_check: u8) {
    let fixture = Fixture(fixture);
    use virtual_fixture_fns as vf;
    if virtual_fixture_node(this, fixture.addr()) != 0 {
        return;
    }
    let home = fixture.home_club();
    let away = fixture.away_club();
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
    top_up(this, home.addr(), staff_id, player_id, fixture.addr(), (fixture.home_side_mark() == 0) as u32);
    let (staff_id, player_id) = virtual_id_block(this, slot, 1, 0);
    covered!("mwg_1 both sides topped up");
    top_up(this, away.addr(), staff_id, player_id, fixture.addr(), ((fixture.away_side_mark() as u8) == 0) as u32); // mwg1: the away side's top-up
    let mut node = [0u8; 6];
    node[..4].copy_from_slice(&(fixture.addr() as u32).to_le_bytes());
    node[4..].copy_from_slice(&(slot as u16).to_le_bytes());
    exe::llist_add_tail(this + vf::FIXTURE_LIST, node.as_mut_ptr() as usize, 6);
    if in_the_check != 0 {
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
pub(crate) unsafe extern "thiscall" fn compare_fill_fixture_squads(this: usize, fixture: usize, slot: u32, in_the_check: u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u8)>(IDX_FILL_FIXTURE_SQUADS)(this, fixture, slot, in_the_check);
    let listed = virtual_fixture_node(this, fixture) != 0;
    SCOREBOARD.record(listed, || format!("fill_fixture_squads(fixture {:x}, slot {:#x}, no_check {}): the fixture is not on the list", fixture, slot, in_the_check));
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
    let fixture = Fixture(fixture);
    use virtual_fixture_fns as vf;
    let home = fixture.home_club().addr();
    let away = fixture.away_club().addr();
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
    let node = virtual_fixture_node(this, fixture.addr());
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
    let fixture = Fixture(fixture);
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
        if !fixture.is_null() && fixture.comp_id() != -2 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), s, fixture.competition().addr(), 1, 0) != 0 {
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
    let club = Club(club);
    {
        type F = unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32;
        let (mut t, mut a, mut r, mut e) = (0usize, 0u8, 0u8, 0u8);
        original_fn::<F>(IDX_COUNT_AVAILABLE_PLAYERS)(this, club.addr(), pass, fixture, &mut t, &mut a, &mut r, &mut e);
        if t != 0 {
            exe::crt_free(t);
        }
    }
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((club.addr() + club::SQUAD, 50 * 4));
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, virtual_squad_fns::BLOCK_SIZE as usize * staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_squad_fns::PLAYER_SIZE, virtual_squad_fns::BLOCK_SIZE as usize * virtual_squad_fns::PLAYER_SIZE));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(IDX_TOP_UP_CLUB_SQUAD);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club.addr(), staff_id, player_id, fixture, pass), || {
        crate::rng::without_counting(|| top_up_club_squad(this, club.addr(), staff_id, player_id, fixture, pass))
    });
    SCOREBOARD.record(equal, || format!("top_up_club_squad(club {:x}, staff {}, player {}, pass {}): original {:x?} rust {:x?}", club.addr(), staff_id, player_id, pass, mo, mr));
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
        s.set_first_name(exe::database_pick_first_name_for_nation(c.nation()));
        s.set_second_name(exe::database_pick_second_name_for_nation(c.nation()));
    } else {
        covered!("mwg_3 club without a nation");
        let first = scaled_crt_rand(read_i32(vp::FIRST_NAME_COUNT));
        s.set_first_name(read_usize(G_FIRST_NAME_TABLE).wrapping_add((first as usize).wrapping_mul(vp::NAME_SIZE)));
        let second = scaled_crt_rand(read_i32(vp::SECOND_NAME_COUNT));
        s.set_second_name(read_usize(G_SECOND_NAME_TABLE).wrapping_add((second as usize).wrapping_mul(vp::NAME_SIZE)));
    }
    s.set_common_name((read_u32(G_COMMON_NAME_TABLE)) as usize);
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
    s.set_age(exe::database_get_staff_age(s, G_CURRENT_DATE as *const u16) as i16);
    write_usize(s.addr() + staff::NATION, c.nation().addr());
    let basque = vp::BASQUE_CLUBS.iter().any(|&k| read_u32(club) == read_u32(k));
    let second_nation = if basque {
        covered!("mwg_3 basque club");
        read_usize(G_NATION_TABLE).wrapping_add(read_u32(KEY_NATION_PAYS_BASQUE) as usize * nation::SIZE)
    } else {
        0
    };
    write_usize(s.addr() + staff::SECOND_NATION, second_nation);
    s.set_int_apps(0);
    s.set_int_goals(0);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION, t, empty);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION + 8, t, empty);
    if exe::database_is_national_team(c) != 0 {
        write_usize(s.addr() + staff::CLUB_JOB, 0);
        write_usize(s.addr() + staff::NATIONAL_JOB, club);
        s.set_job_for_club(0);
        s.set_job_for_nation(0xb);
    } else {
        write_usize(s.addr() + staff::CLUB_JOB, club);
        write_usize(s.addr() + staff::NATIONAL_JOB, 0);
        s.set_job_for_club(0xb);
        s.set_job_for_nation(0);
    }
    let joined = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year - 1) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_CLUB, t, joined);
    let expires = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year + 6) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_EXPIRES_CLUB, t, expires);
    s.set_value((0) as usize);
    s.set_wage((c.reputation() as i32 / 5) as usize);
    s.set_adaptability(((scaled_crt_rand(10) as u8).wrapping_add(1)) as i8);
    for f in [staff::AMBITION, staff::DETERMINATION, staff::LOYALTY, staff::PRESSURE, staff::PROFESSIONALISM, staff::SPORTSMANSHIP, staff::TEMPERAMENT] {
        write_u8(s.addr() + f, 10);
    }
    s.set_classification(2);
    write_usize(s.addr() + staff::PLAYER, p.addr());
    s.set_staff_preferences(0);
    write_usize(s.addr() + staff::NON_PLAYER, 0);
    s.set_squad_selected_for(0);
    let national = exe::database_is_national_team(c) != 0;
    let mark = match (national, reserve != 0) {
        (true, true) => 8,
        (true, false) => 4,
        (false, true) => 2,
        (false, false) => 1,
    };
    s.set_playing_squad(mark);
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
    let club = Club(club);
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_player_consts::PLAYER_SIZE, virtual_player_consts::PLAYER_SIZE));
    regions.push((club.addr() + club::SQUAD, 50 * 4));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32)>(IDX_CREATE_VIRTUAL_PLAYER);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club.addr(), group, staff_id, player_id, youngster, reserve, squad_number), || {
        crate::rng::without_counting(|| create_virtual_player(this, club.addr(), group, staff_id, player_id, youngster, reserve, squad_number))
    });
    SCOREBOARD.record(equal, || format!("create_virtual_player(club {:x}, group {:#x}, staff {}, player {}, youngster {}, reserve {}, number {}): original {:x?} rust {:x?}",
        club.addr(), group, staff_id, player_id, youngster, reserve, squad_number, mo, mr));
}

/// The first staff and player ids of the block of 25 virtual records of a fixture side or of a viewed club slot
/// (`VIRTUAL_STAFF::get_id_block`, still the exe's): `slot` is passed as the original's whole register.
pub(crate) unsafe fn virtual_id_block(this: usize, slot: u32, is_fixture: u32, is_home: u32) -> (u32, u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32, *mut u32) = core::mem::transmute(virtual_fixture_fns::GET_ID_BLOCK);
    let (mut staff_id, mut player_id) = (0u32, 0u32);
    f(this, slot, is_fixture, is_home, &mut staff_id, &mut player_id);
    (staff_id, player_id)
}

/// The fixture's node in the VIRTUAL_STAFF fixture list, or 0: the cursor goes back to the head first.
pub(crate) unsafe fn virtual_fixture_node(this: usize, fixture: usize) -> usize {
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
pub(crate) unsafe fn drop_club_virtual_players(club: Club) {
    for i in 0..50 {
        let s = club.squad(i);
        if s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT) {
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 1, 1);
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 0, 1);
            club.set_squad(i, 0);
        }
    }
}

/// `create_virtual_player` through the exe (the detour runs its comparer in comparison mode).
pub(crate) unsafe fn virtual_create(this: usize, club: Club, group: u8, staff_id: u32, player_id: u32, youngsters: u32, reserve: u32, squad_number: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(virtual_squad_fns::CREATE_VIRTUAL_PLAYER);
    f(this, club.addr(), group as u32, staff_id, player_id, youngsters, reserve, squad_number as u32);
}

/// Whether a template player record plays the position group (`get_player_position_group` numbering; 15 or more,
/// and exactly 20 for a goalkeeper).
pub(crate) unsafe fn template_fits_group(t: PlayerSheet, group: i32) -> bool {
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
